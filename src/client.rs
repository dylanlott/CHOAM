use num_bigint::BigInt;
use rand::Rng;
use tonic::transport::Channel;
use zkp_auth::{auth_client::AuthClient, RegisterRequest};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

const PRIME: u32 = 53239;
const GENERATOR: u32 = 2;

pub struct ZKPAuthClient {
    client: AuthClient<Channel>,
}

impl ZKPAuthClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AuthClient::connect(url).await?;
        Ok(ZKPAuthClient { client })
    }

    pub async fn send_register_request(&mut self, username: String, x: i64) {
        println!("sending registration request for {}", username);
        let (y1, y2) = setup_y1_y2(BigInt::from(x));

        let _y1: u64 = y1.bits().clone();
        let _y2: u64 = y2.bits().clone();

        let req = tonic::Request::new(RegisterRequest {
            user: username.to_string(),
            y1: _y1 as i64,
            y2: _y2 as i64,
        });
        let resp = self.client.register(req).await;
        println!("REGISTRATION RESPONSE: {:?}", resp);
    }
}

fn setup_y1_y2(x: BigInt) -> (BigInt, BigInt) {
    println!("setting up y1 & y2");
    let generator = BigInt::from(GENERATOR);
    let prime = BigInt::from(PRIME);
    let random = BigInt::from(rand::thread_rng().gen_range(1u32..100));
    let y1 = generator.modpow(&x, &prime);
    let y2 = generator.modpow(&random, &prime);
    (y1, y2)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CHOAM Weapon Response Authentication System - Starting Authentication Process");

    let zkp_client = ZKPAuthClient::new("://[::1]:50051".to_string());

    println!("established connection with CHOAM host");

    let mut client = zkp_client.await?;
    
    let _registration_resp = client 
        .send_register_request("shakezula".to_string(), 42i64).await;

    println!("registration respoinse: {:?}", _registration_resp);

    Ok(())
}
