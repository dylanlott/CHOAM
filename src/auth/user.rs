use num_bigint::BigInt;

#[derive(Default, Debug, Clone)]
pub struct User {
    pub username: String,
    pub y1: BigInt,
    pub y2: BigInt,
    pub challenge: i64,
}