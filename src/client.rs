use num_bigint::{BigInt, ToBigInt};
use rand::Rng;
use tonic::{transport::Channel, Request, Response, Status};
use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerResponse, AuthenticationChallengeResponse,
    RegisterRequest,
};

use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest};

mod auth;

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

const PRIME: u32 = 53239;
const GENERATOR: u32 = 2;

pub struct ZKPAuthClient {
    client: AuthClient<Channel>,
    user: auth::user::User,
    token: String,
}

impl ZKPAuthClient {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AuthClient::connect(url).await?;
        let user = auth::user::User {
            ..Default::default()
        };
        Ok(ZKPAuthClient {
            client,
            user,
            token: "".to_string(),
        })
    }

    pub async fn send_register_request(&mut self, username: String, x: i64) -> (i64, i64, i64) {
        println!("sending registration request for {}", username);
        let (y1, y2, random) = setup_y1_y2(BigInt::from(x));

        println!("y1: {} - y2: {} - random: {}", y1, y2, random);

        // TODO Handle signs in all of these 
        let (_, y1b) = y1.to_bytes_le();
        let _y1 = vec_to_i64(y1b);
        
        let (_, y2b) = y2.to_bytes_le();
        let _y2 = vec_to_i64(y2b);
        
        let (_, rb) = random.to_bytes_le();
        let _random = vec_to_i64(rb);

       println!("_y1: {} - _y2: {} - _random: {}", _y1, _y2, _random);

        // self.user.random = random;
        // self.user.username = username.to_string();
        // self.user.y1 = BigInt::from(y1);
        // self.user.y2 = BigInt::from(y2);

        let req = Request::new(RegisterRequest {
            user: username.to_string(),
            ..Default::default()
        });

        match self.client.register(req).await {
            Ok(resp) => {
                let msg = resp.into_inner();
                println!("received successful registration request: {:?}", msg)
            }
            Err(status) => {
                eprintln!("error: failed to register: {}", status.message())
            }
        }

        (_y1, _y2, _random)
    }

    pub async fn send_auth_challenge(
        &mut self,
        r1: i64,
        r2: i64,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("sending authentication challenge");

        let req = Request::new(AuthenticationChallengeRequest {
            user: "shakezula".to_string(),
            r1,
            r2,
        });

        let resp = self.client.create_authentication_challenge(req).await;

        println!("received authentication challenge: {:?}", resp);

        return resp;
    }

    pub async fn send_auth_answer(
        &mut self,
        s: BigInt,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        println!("sending authentication challenge");

        let _s = s.bits().clone();
        let req = AuthenticationAnswerRequest {
            auth_id: self.user.username.clone(),
            s: _s as i64,
        };

        return self.client.verify_authentication(req).await;
    }
}

fn setup_y1_y2(x: BigInt) -> (BigInt, BigInt, BigInt) {
    println!("initializing knowledge generation");
    let generator = BigInt::from(GENERATOR);
    let prime = BigInt::from(PRIME);
    let random = BigInt::from(rand::thread_rng().gen_range(1u32..100));
    let y1 = generator.modpow(&x, &prime);
    let y2 = generator.modpow(&random, &prime);
    (y1, y2, random)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("**WARS**: Weapon Authentication Response System --- starting up");
    let secret = 42i64; // TODO get from CLI prompt
    let zkp_client = ZKPAuthClient::new("://[::1]:50051".to_string());

    println!("established connection with CHOAM host");

    let mut client = zkp_client.await?;
    let (r1, r2, random) = client
        .send_register_request("shakezula".to_string(), secret)
        .await;

    println!("registration successful: r1 {} - r2 {} - RANDOM: {}", r1, r2, random);
    println!("requesting authentication challenge");

    match client.send_auth_challenge(r1, r2).await {
        Ok(resp) => {
            let msg = resp.into_inner();
            println!( "computing response from challenge parameers: {}", msg.c);

            let _r = client.user.random.clone();
            let challenge = msg.c.clone();
            println!("RANDOM IS: {} - challenge: {}", _r, challenge);
            let answer = _r + challenge * secret;

            println!("computed challenge answer: {}", answer);

            // respond with the answer to the random challenge
            match client.send_auth_answer(answer).await {
                Err(err) => {
                    eprintln!("failed to authenticate: {}", err.message())
                }
                Ok(resp) => {
                    let msg = resp.into_inner();
                    println!("authentication successful ✅");
                    client.token = msg.session_id;
                }
            }
        }
        Err(status) => {
            eprintln!("auth challenge error: {}", status.message())
        }
    }

    Ok(())
}

fn vec_to_i64(bytes: Vec<u8>) -> i64 {
    let mut result: i64 = 0;

    for &byte in bytes.iter() {
        result = (result << 8) | (byte as i64);
    }

    result
}
