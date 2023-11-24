use auth::user::User;
use auth::zkp_auth::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Result, Status};

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
        println!("received registration request: {}", req.clone().user);

        self.add_user(
            req.user.to_string(),
            User {
                username: req.user.to_string(),
                y1: req.y1.into(),
                y2: req.y2.into(),
                challenge: 0,
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

        // TODO handle r1 and r2 

        let username = req.user.clone();
        println!("received authentication challenge request");

        match self.get_user(req.user).await {
            Some(user) => {
                let challenge: i64 = rand::thread_rng().gen_range(1u32..1000).into();
                // lock users collection & update user's challenge value for later use during verification
                self.users.lock().await.insert(username.clone(), User { 
                    username: username.clone(), 
                    y1: user.y1,
                    y2: user.y2,
                    challenge,
                });
                // respond with the random challenge for answer & verification from client
                Ok(Response::new(AuthenticationChallengeResponse {
                    auth_id: user.username,
                    c: challenge,
                }))
            }
            None => {
                Err(Status::not_found("User not found"))
            }
        }
    }
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        panic!("TODO")
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("**CHOAM**: Chaum-Pedersen Heavy Orbital Assault Machine --- starting up");
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
