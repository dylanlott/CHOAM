use num_bigint::BigUint;
use rand::Rng;

use crate::auth::auth::AuthService;

mod auth;

fn main() {
    let _svc = auth::auth::AuthService{};

    println!("** Chaum-Pedersen Sigma Protocol **");

    // first, set a very large prime number 
    let _p = BigUint::parse_bytes(b"29681414807193618078300503894496900591389075175848196575852013250824428361350773898226570528483777365493346325763613640645922124183625513419110102886887985133189255562298626741602809789062309223691579812700352910367842967695657459861851531211067775969455525985867918262546293597343901574119146998553353507903456640060371193924396346542093123090019460491060682335047691431322399204447393320168687318900159478464378155315345832828943811843048146020195167770547183961910489837004875712120862996001092849872095792202183089679972549487934485916851744727433705020777679383830220041324931819169629506145568195857141649730651", 10).unwrap();

    // then declare a generator
    // let's use 2 for simplicity but normally generators have more thought put into them.
    // generator must not be a multiple of _p.
    let _g = BigUint::from(2u32);

    // x will actually be our "password" here. the "random logarithm" I think.
    let _x: BigUint = BigUint::from(42u32);
    
    // h = g^x mod p
    let _h = _g.modpow(&_x, &_p);
   
    // Now the prover has their logarithm _h. 
    
    // Next, the prover assembles a random _r for use in challenges later
    let _r = BigUint::from(rand::thread_rng().gen_range(1u32..100));

    // remember: _g and _p are public information here. _r is the prover's challenge.
    // prover generates a new _t from their previous _r
    let _t: BigUint = _g.modpow(&_r, &_p);
    // > note: t is what you would save as the _verifier_ 

    // now, verifier (server) would send a random challenge, c 
    let _challenge = BigUint::from(rand::thread_rng().gen_range(0u32..10));

    // prover now takes the challenge and computes an answer from 
    // their own previous _r and the challenge _c
    let _answer = &_r + &_challenge * &_x;
    println!("answer: {}", _answer);

    // next, the verifier checks the answer: 
    let left = _g.modpow(&_answer, &_p);
    let right = (&_t * &_h.modpow(&_challenge, &_p)) % &_p; 

    if left == right {
        println!("holy shit, this is zero knowledge in action: {}=={}", left, right)
    } else {
        println!("it didn't work, check your shit scrub.")
    }
}
