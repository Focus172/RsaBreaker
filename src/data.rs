use rand::random;

pub struct KeySet {
    pub pub_key: (u64, u64),
    priv_key: (u64, u64),
    key_factor_1: u64,
    key_factor_2: u64,
}

impl KeySet {
    pub fn new() -> KeySet {
        let p = generate_prime();
        let q = generate_prime();
        
        let n = p * q;
        let phi = (p-1)*(q-1);
        // let e = //a prime number such that the gcd of it and phi is 1
        let e = prime_bounded_range(&phi); 
        
        let d = inverse_gcd(&e, &phi);
        
        KeySet {
            pub_key: (n, e),
            priv_key: (n, d),
            key_factor_1: p,
            key_factor_2: q
        }
    }

    /// this funtion destorys the object returning the private key
    /// this is meant to be a strange way in which the data can only be acsessed once  
    fn get_priv_key(self) -> (u64, u64) {
        let key = self.priv_key;
        //self.free(); 
        return key;
    }

    fn print(&self) {
       println!("Public keys: {} {}", self.pub_key.0, self.pub_key.1);
       println!("Private keys: {} {}", self.priv_key.0, self.priv_key.1);
    }
}

pub struct Data {}

impl Data {
    pub fn new() -> Data {
        Data {}
    }

    pub fn next(&self) -> Option<KeySet> {
        if random::<u8> as u8 == 1 {
            None
        } else {
            Some(KeySet::new())
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
            return r
        }
    }
}

fn inverse_gcd(e: &u64, phi: &u64) -> u64 {
    let mut r = 1;
    loop {
        r += e;
        if r % phi == 1 {
            return r
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

