extern crate num;
extern crate num_traits;
extern crate rand;

use num_traits::Pow;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn primes(p: u64) -> Vec<u64> {
    let mut prime: Vec<bool> = vec![true; p as usize];
    let mut r = 2;
    while r * r <= p {
        if prime[r as usize] {
            for i in (r * r..p + 1).step_by(r as usize) {
                prime[i as usize] = false;
            }
        }
        r += 1;
    }
    let mut return_vec = Vec::new();
    for (i, &elem) in prime.iter().skip(2).enumerate() {
        if elem {
            return_vec.push((i + 2) as u64);
        }
    }
    println!("{:?}", return_vec);
    return_vec
}

pub fn modular_pow(a: u64, b: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut c = 1;
    for _x in 0..b {
        c = (c * a) % m;
    }
    c
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    *(primes(p).choose(&mut rng).unwrap())
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // Pow::pow(b_pub, a as u32) % p
    modular_pow(b_pub, a, p)
}
