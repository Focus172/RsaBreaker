use rand::{random, rngs::ThreadRng};

use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct KeySet {
    pub pub_key: RsaPublicKey,
    priv_key: RsaPrivateKey,
}

const BITS: usize = 2048;

impl KeySet {
    pub fn new(rng: &mut ThreadRng) -> KeySet {
        let priv_key = RsaPrivateKey::new(rng, BITS).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        KeySet { pub_key, priv_key }
    }
}

// fn print(&self) {
//    println!("Public keys: {} {}", self.pub_key.0, self.pub_key.1);
//    println!("Private keys: {} {}", self.priv_key.0, self.priv_key.1);
// }
pub struct Data {
    rng: ThreadRng,
}

impl Data {
    pub fn new() -> Data {
        let mut rng = rand::thread_rng();
        Data { rng }
    }

    pub fn next(&mut self) -> Option<KeySet> {
        if random::<u8>() as u8 == 1 {
            None
        } else {
            Some(KeySet::new(&mut self.rng))
        }
    }
}

//uses the Primality test to generate a large prime number
fn generate_prime() -> u64 {
    loop {
        let num = random::<u64>();
        if is_prime(num) {
            return num;
        }
    }
}

fn prime_bounded_range(phi: &u64) -> u64 {
    loop {
        let r = generate_prime();
        if r < *phi {
            return r;
        }
    }
}

fn inverse_gcd(e: &u64, phi: &u64) -> u64 {
    let mut r = 1;
    loop {
        r += e;
        if r % phi == 1 {
            return r;
        }
    }
}

/// Basic implementation of primality test
/// Adpated from: https://en.wikipedia.org/wiki/Primality_test#Python
fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n < 2 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;

    // let limit = sqrt(n)
    // for i in 5..limit.filter(|n| n % 6 != 0)

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
