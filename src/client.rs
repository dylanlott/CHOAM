use num_bigint::BigInt;
use rand::Rng;
use std::convert::TryInto;
use tonic::{transport::Channel, Request, Response, Status};
use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerResponse, AuthenticationChallengeResponse,
    RegisterRequest,
};
use tracing::{error, info};

use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest};

mod auth;

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

// public values used for the calculation of the logarithms used in the swap.
const PRIME: u32 = 53239;
const GENERATOR: u32 = 2;

// ZKPAuthClient holds a reference to the client, the user's parameters,
// and the token, if any has been acquired.
pub struct ZKPAuthClient {
    client: AuthClient<Channel>,
    user: auth::user::User,
    token: String,
}

// ZKPAuthClient implements the Chaum-Pedersen Orbital Authentication Machine
// protocol from the client perspective.
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

    // send_register_request sends a registration request for the given 
    // username and secret value. it sets up y1 and y2 parameters as well
    // as the random value used for computing a challenge.
    pub async fn send_register_request(&mut self, username: String, x: i64) -> (i64, i64, i64) {
        info!("sending registration request for {}", username);

        // calculate y1, y2, and random value from the provided secret
        let (y1, y2, random) = setup_y1_y2(BigInt::from(x));

        // set internal values for later user
        self.user.random = BigInt::from(random);
        self.user.username = username.clone();
        self.user.y1 = BigInt::from(y1);
        self.user.y2 = BigInt::from(y2);

        // format a registration request
        let req = Request::new(RegisterRequest {
            user: username.to_string().clone(),
            y1,
            y2,
        });

        // send the registration request and handle errors if any occur
        match self.client.register(req).await {
            Ok(_) => {
                info!("registration successful ✅")
            }
            Err(status) => {
                error!("failed to register {}: {}", username, status.message())
            }
        }

        (y1, y2, random)
    }

    // send_auth_challenge gets a new challenge from the verifier which
    // allows a new computation of `s`
    pub async fn send_auth_challenge(
        &mut self,
        r1: i64,
        r2: i64,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        info!("sending authentication challenge");

        let req = Request::new(AuthenticationChallengeRequest {
            user: "shakezula".to_string(),
            r1,
            r2,
        });

        let resp = self.client.create_authentication_challenge(req).await;

        info!("received authentication challenge");

        return resp;
    }

    pub async fn send_auth_answer(
        &mut self,
        s: BigInt,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        info!("sending authentication challenge");

        let result: Result<i64, _> = s.try_into();
        match result {
            Ok(i) => { 
                self.user.challenge = i;
                let req = AuthenticationAnswerRequest {
                    auth_id: self.user.username.clone(),
                    s: i,
                };

                return self.client.verify_authentication(req).await;
            }
            Err(e) => panic!("Failed to convert: {:?}", e),
        }
    }
}

// setup_y1_y1 returns the y1 and y2 values along with the random seed used to 
// generate them.
fn setup_y1_y2(x: BigInt) -> (i64, i64, i64) {
    info!("initializing knowledge generation");
    let generator = BigInt::from(GENERATOR);
    let prime = BigInt::from(PRIME);
    let random = BigInt::from(rand::thread_rng().gen_range(1u32..100));
    let y1 = generator.modpow(&x, &prime);
    let y2 = generator.modpow(&random, &prime);

    let mut _y1: i64 = 0;
    let mut _y2: i64 = 0;
    let mut _random: i64 = 0;

    let result: Result<i64, _> = y1.try_into();
    match result {
        Ok(i) => { _y1 = i },
        Err(e) => panic!("Failed to convert: {:?}", e),
    }

    let result: Result<i64, _> = y2.try_into();
    match result {
        Ok(i) => _y2 = i,
        Err(e) => panic!("Failed to convert: {:?}", e),
    }

    let result: Result<i64, _> = random.try_into();
    match result {
        Ok(i) => _random = i,
        Err(e) => panic!("Failed to convert: {:?}", e),
    }

    (_y1, _y2, _random)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("**CHOAM**: Authentication client starting up ⬆️");

    let zkp_client = ZKPAuthClient::new("://[::1]:50051".to_string());

    let secret = 42i64; // TODO get from CLI prompt
    
    info!("established connection with CHOAM host");

    let mut client = zkp_client.await?;
    let (r1, r2, _) = client
        .send_register_request("shakezula".to_string(), secret)
        .await;

    info!("requesting authentication challenge");

    match client.send_auth_challenge(r1, r2).await {
        Ok(resp) => {
            let msg = resp.into_inner();
            
            info!("computing response from challenge parameters");

            let random = client.user.random.clone();
            let challenge = msg.c.clone();
            let answer = random + challenge * secret;

            info!("computed challenge answer");

            // respond with the answer to the random challenge
            match client.send_auth_answer(answer).await {
                Err(err) => {
                    error!("failed to authenticate: {}", err.message())
                }
                Ok(resp) => {
                    let msg = resp.into_inner();
                    info!("authentication successful 🔑");
                    client.token = msg.session_id;
                }
            }
        }
        Err(status) => {
            error!("auth challenge error: {}", status.message())
        }
    }

    Ok(())
}