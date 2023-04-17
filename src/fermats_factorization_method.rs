use num_bigint::BigUint;

pub fn get_list_of_prime_factors(n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();
    let one: BigUint = BigUint::from(1u8);

    let mut a: BigUint = n.sqrt();

    if &a * &a == n {
        prime_list.push(a.clone());
        prime_list.push(a);
        return prime_list;
    }

    loop {
        let aa = &a * &a;
        let b1: BigUint = if aa >= n { aa - &n } else { &n - aa };
        let b: BigUint = b1.sqrt();
        if &b * &b == b1 {
            if a >= b {
                prime_list.push(&a - &b);
                prime_list.push(&a + &b);
            } else {
                prime_list.push(&b - &a);
                prime_list.push(&a + &b);
            }
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
