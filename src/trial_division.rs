use num_bigint::BigUint;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();
    let mut f = BigUint::from(2usize);

    let zero: BigUint = BigUint::from(0usize);
    let one: BigUint = BigUint::from(1usize);

    while &n > &one {
        if &n % &f == zero {
            prime_list.push(f.clone());
            n = &n / &f;
        } else {
            f += &one;
        }
    }
    prime_list
}

pub fn get_lowest_prime_factor(mut n: BigUint) -> BigUint{

    let mut prime_list: Vec<BigUint> = Vec::new();
    let mut f = BigUint::from(2usize);

    let zero: BigUint = BigUint::from(0usize);
    let one: BigUint = BigUint::from(1usize);

    while &n > &one {
        if &n % &f == zero {
            prime_list.push(f.clone());
            n = &n / &f;
        } else {
            f += &one;
        }
    }
    prime_list[0].clone()
}
