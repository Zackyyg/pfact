use num_bigint::BigUint;
use num_integer::Integer;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();
    let one: BigUint = BigUint::from(1u8);
    let two: BigUint = BigUint::from(2u8);

    while &n > &one {
        let mut a: BigUint = BigUint::from(2u8);
        let mut i: BigUint = BigUint::from(2u8);
        let mut bound: BigUint = BigUint::from(10u8);

        loop {
            a = a.modpow(&i, &n);
            a = if a > one { &a - &one } else { BigUint::from(0u8) };

            let d: BigUint = a.gcd(&n);

            if d > one {
                n /= &d;
                prime_list.push(d);
                break;
            }

            if i > bound {
                bound *= &two;
                a = two.clone();
                i = two.clone();
            }
            i += &one;
        }
    }

    prime_list.sort();
    prime_list
}
