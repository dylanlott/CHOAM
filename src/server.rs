use auth::user::{User, Claims};
use auth::zkp_auth::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};
use num_bigint::BigInt;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Result, Status};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use tracing_subscriber;
use tracing::{info, error};
mod auth;

const PRIME: u32 = 53239;
const GENERATOR: u32 = 2;

pub struct AuthService {
    users: Arc<Mutex<HashMap<String, User>>>,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        info!("received registration request: {}", req.clone().user);

        self.add_user(
            req.user.to_string(),
            User {
                username: req.user.to_string(),
                y1: req.y1.into(),
                y2: req.y2.into(),
                challenge: 0,
                random: BigInt::from(0i64),
            },
        )
        .await;

        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let req = request.into_inner();
        let username = req.user.clone();

        info!("received authentication challenge request for {}", username);

        match self.get_user(req.user).await {
            Some(user) => {
                let challenge: i64 = rand::thread_rng().gen_range(1u32..1000).into();
                // lock users collection & update user's challenge value for later use during verification
                self.users.lock().await.insert(
                    username.clone(),
                    User {
                        username: username.clone(),
                        y1: user.y1,
                        y2: user.y2,
                        challenge,
                        ..Default::default()
                    },
                );

                // respond with the random challenge for answer & verification from client
                Ok(Response::new(AuthenticationChallengeResponse {
                    auth_id: user.username,
                    c: challenge,
                }))
            }
            None => Err(Status::not_found("User not found")),
        }
    }
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let req = request.into_inner();
        let username = &req.auth_id.to_string();

        // match a user
        match self.get_user(username.clone()).await {
            Some(user) => {
                let _generator = BigInt::from(GENERATOR);
                let _prime = &BigInt::from(PRIME);
                let challenge = &BigInt::from(user.challenge);
                let answer = &BigInt::from(req.s);

                info!("challenge issued for {}", username);
                
                // construct the equation
                let left = _generator.modpow(answer, _prime);
                let right = (user.clone().y2 * user.clone().y1.modpow(challenge, _prime)) % _prime;

                // compare equality of left to right
                if left == right {
                    match generate_jwt() {
                        Ok(token) => {
                            info!("generated authorization token for {}", username.clone());
                            let resp = Response::new(AuthenticationAnswerResponse { 
                                session_id: token,
                            });
                            Ok(resp)
                        },
                        Err(_) => {
                            Err(Status::internal("failed to generate jwt"))
                        },
                    }
                } else {
                    error!("🚩 failed validation: {}", username.clone());
                    Err(Status::not_found("User not found"))
                }
            }
            None => {
                error!("failed to find user: {}", username.clone());
                Err(Status::permission_denied("denied"))
            }
        }
    }
}

impl AuthService {
    async fn add_user(&self, username: String, user: User) {
        let mut users = self.users.lock().await;
        users.insert(username, user);
    }

    // Method to retrieve a user by username
    async fn get_user(&self, username: String) -> Option<User> {
        let users = self.users.lock().await;
        let found = users.get(&username);
        found.cloned()
    }
}

fn generate_jwt() -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims{
        sub: "WARS".to_owned(),
        company: "CHOAM".to_owned(),
        exp: expiration as usize,
    };

    let secret = "your_secret_key"; // TODO load from env
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("**CHOAM**: Chaum-Pedersen Heighliner Orbital Assault Machine --- starting up ⬆️");

    let addr = "[::1]:50051".parse()?;
    let svc = AuthService {
        users: Arc::new(Mutex::new(HashMap::new())),
    };
    Server::builder()
        .add_service(AuthServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
