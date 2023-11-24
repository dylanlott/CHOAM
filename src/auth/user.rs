use num_bigint::BigUint;

#[derive(Default, Debug)]
pub struct User {
    username: String,
    h: BigUint,
    r: BigUint,
    t: BigUint,
}