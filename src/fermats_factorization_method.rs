use num_bigint::BigUint;

pub fn get_list_of_prime_factors(n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();
    let one: BigUint = BigUint::from(1u8);

    let mut a: BigUint = n.sqrt() + &one;

    if &a * &a == n {
        prime_list.push(a.clone());
        prime_list.push(a);
        return prime_list;
    }

    loop {
        let b1 = a.pow(2) - &n;
        let b: BigUint = b1.sqrt();
        if &b * &b == b1 {
            prime_list.push(&a - &b);
            prime_list.push(&a + &b);
            break;
        } else {
            a = &a + &one;
        }
    }
    prime_list
}








// FermatFactor(N): // N should be odd
//     a ← ceiling(sqrt(N))
//     b2 ← a*a - N
//     repeat until b2 is a square:
//         a ← a + 1
//         b2 ← a*a - N
//      // equivalently:
//      // b2 ← b2 + 2*a + 1
//      // a ← a + 1
//     return a - sqrt(b2) // or a + sqrt(b2)
