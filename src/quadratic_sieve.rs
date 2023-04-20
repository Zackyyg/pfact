use num_bigint::BigUint;
use num_traits::FromPrimitive;
use rug::{Float, ops::Pow};


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();

    let b: Float = get_b(&n);

    let b_usize = b.to_integer().expect("B can't be converted to Integer").to_usize().expect("B is too large to fit in a usize");

    let factor_base: Vec<usize> = find_primes(b_usize);




    prime_list.sort();
    prime_list
}

fn get_b(n: &BigUint) -> Float {
    let precision = 4096; // Set the precision to handle large numbers

    let zero = Float::with_val(precision, 0);
    let one_half = Float::with_val(precision, 0.5);
    let e = Float::with_val(precision, Float::e());

    let n_float = Float::with_val(precision, n);
    let log_n = n_float.clone().ln();
    let log_log_n = log_n.clone().ln();

    let o1 = zero.clone(); // You can set o(1) to a value approaching 0, or just set it to 0

    let exponent = (one_half + o1) * log_n * log_log_n.sqrt();

    e.pow(&exponent)
}

fn find_primes(limit: usize) -> Vec<usize> {

    let mut prime_list: Vec<usize> = Vec::new();

    let mut sieve: Vec<bool> = vec![true; &limit + 1];

    let mut p: usize = 2;
    while p * p <= limit {
        if sieve[p] {
            let mut i: usize = p * p;
            while i <= limit {
                let exponent = (p - 1) / 2;
                let result = mod_exp(i ,exponent, p);

                if result == 1 {
                    sieve[i] = false;
                }
                i += p;
            }
        }
        p += 1;
    }

    for i in 2..limit {
        if sieve[i] == true {
            prime_list.push(i);
        }
    }

    prime_list
}

fn mod_exp(base: usize, exponent: usize, modulus: usize) -> usize {
    let mut result: usize = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }

    result
}

