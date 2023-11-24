use auth::user::User;
use auth::zkp_auth::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};
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
            },
        )
        .await;

        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        panic!("TODO")
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
