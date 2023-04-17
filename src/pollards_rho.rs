use num_bigint::BigUint;
use num_integer::Integer;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();
    let one: BigUint = BigUint::from(1u8);


    while &n >= &one {
        let mut x: BigUint = BigUint::from(2u8);
        let mut y: BigUint = x.clone();
        let mut d: BigUint = BigUint::from(1u8);

        while &d == &one{
            x = g(&x, &n);

            y = g(&g(&y, &n), &n);

            let abs_diff = if x > y { &x - &y } else { &y - &x };
            d = abs_diff.gcd(&n);
        }

        if d != n {
            prime_list.push(d.clone());
            n /= d;
        } else {
            prime_list.push(n);
            break;
        }
    }
    prime_list.sort();
    prime_list
}

fn g(x: &BigUint, n: &BigUint) -> BigUint{
    (BigUint::pow(x, 2) + &BigUint::from(1u8)) % n
}
