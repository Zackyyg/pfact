use num_bigint::BigUint;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();

    let zero: BigUint = BigUint::from(0usize);
    let one: BigUint = BigUint::from(1usize);
    let mut f: BigUint = BigUint::from(2usize);

    while &n > &one {
        if &n % &f == zero {
            prime_list.push(f.clone());
            n /= &f;
        } else {
            f += &one;
        }
    }
    prime_list
}
