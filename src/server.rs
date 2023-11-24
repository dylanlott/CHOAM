use auth::zkp_auth::{auth_server::{Auth, AuthServer}, RegisterResponse, RegisterRequest, AuthenticationChallengeRequest, AuthenticationChallengeResponse, AuthenticationAnswerRequest, AuthenticationAnswerResponse};
use tonic::{transport::Server, Status, Result, Response};

mod auth;

pub struct AuthService {}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(&self, request: tonic::Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        panic!("TODO")
    }
    async fn create_authentication_challenge(&self, request: tonic::Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        panic!("TODO")
    }
    async fn verify_authentication(&self, request: tonic::Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status>{
        panic!("TODO")
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:50051".parse()?;
    let svc = AuthService{ };
    Server::builder()
        .add_service(AuthServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}