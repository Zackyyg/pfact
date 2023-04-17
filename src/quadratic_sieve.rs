use num_bigint::BigUint;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();

    // find b smoth numbers
    //let b: BigUint = b_smooth_number(&n);

    // solve kernels of a matrix

    prime_list.sort();
    prime_list
}

// fn b_smooth_number(n: BigUint) -> BigUint {
//     let mut b: BigUint = BigUint::from(1u32);
//     let mut i: BigUint = BigUint::from(2u32);
//     while b < n {
//         b = b * i;
//         i = i + BigUint::from(1u32);
//     }
//     b
// }
