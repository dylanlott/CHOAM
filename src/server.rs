use auth::user::User;
use auth::zkp_auth::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};
use num_bigint::BigUint;
use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Result, Status};

mod auth;

const PRIME: u32 = 53239;
const GENERATOR: u32 = 2;

pub struct AuthService {
    users: HashMap<String, auth::user::User>,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        panic!("TODO")
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
    fn add_user(&mut self, username: String, user: User) {
        self.users.insert(username, user);
    }

    // Method to retrieve a user by username
    fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc = AuthService {
        users: HashMap::new(),
    };
    Server::builder()
        .add_service(AuthServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
