use num_bigint::BigUint;

#[derive(Default, Debug)]
pub struct User {
    H: BigUint,
    R: BigUint,
    T: BigUint,
}