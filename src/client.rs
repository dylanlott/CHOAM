use tonic::transport::Channel;
use zkp_auth::{auth_client::AuthClient, RegisterRequest};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

pub struct ZKPAuthClient {
    client: AuthClient<Channel>
}

impl ZKPAuthClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AuthClient::connect(url).await?;
        Ok(ZKPAuthClient { client })
    }

    pub async fn send_register_request() {

        let req = tonic::Request::new(RegisterRequest{
            user: "foo".to_string(),
            y1: 0, // TODO 
            y2: 0, // TODO
        });

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _zkp_client = ZKPAuthClient::new("[::1]:50051".to_string());
    Ok(())
}