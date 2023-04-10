use num_bigint::{BigUint, RandBigInt};
use num_traits::Num;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();

    prime_list
}

fn modular_pow(mut base: BigUint, mut exponent: BigUint, modulus: BigUint) -> BigUint {
        let mut result = BigUint::from(1u8);
        let one = BigUint::from(1u8);
        let zero = BigUint::from(0u8);

        while (&exponent > zero) {
            if (&exponent % BigUint::from(2u8) == one)
                result = (&result * &base) % &modulus;

            exponent = &exponent / BigUint::from(2u8);
            base = (&base * &base) % &modulus;
        }
        result;
    }



pub fn get_lowest_prime_factor(mut n: BigUint) -> BigUint{
    let one: BigUint = BigUint::from(1u8);
    let two: BigUint = BigUint::from(2u8);

    let mut prime_list: Vec<BigUint> = Vec::new();

    if (&n == &one){
        throw new Exception("n is 1");
    }

    if (&n % &two == 0){
        return two;
    }

    /* we will pick from the range [2, N) */
    let low = 2.to_bigint().unwrap();
    let x = rng.gen_bigint_range(&low, &n);
    let y = x.clone();

    let c = rng.gen_bigint_range(&low, &n);

    let mut d = one.clone();

    while &d == one {
        x = (modular_pow(x, BigUint::from(2u8), n) + &c + &n) % &n;

        y = (modular_pow(&y, BigUint::from(2u8), n) + &c + &n) % &n;


        y = (&y * &y + &c) % &n;
        d = gcd(&(&x - &y).abs(), &n);
    }


}
