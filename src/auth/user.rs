use num_bigint::BigInt;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone)]
pub struct User {
    pub username: String,
    pub y1: BigInt,
    pub y2: BigInt,
    pub random: BigInt,
    pub challenge: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user identifier)
    pub company: String,
    pub exp: usize, // expiration time
}