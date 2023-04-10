use num_bigint::BigUint;
use num_traits::Num;

use pfact::trial_division::{get_list_of_prime_factors, get_lowest_prime_factor};


#[test]
fn test_get_list_of_prime_factors() {
    let expected_factors = vec![
        BigUint::from(7usize)
    ];
    let prime_factors = get_list_of_prime_factors(BigUint::from_str_radix("7", 10).expect("Failed to convert string to BigUint"));
    assert_eq!(prime_factors, expected_factors);
}

#[test]
fn test_get_lowest_prime_factor() {
    let expected_lowest_factor = BigUint::from(7usize);
    let lowest_prime_factor = get_lowest_prime_factor(BigUint::from_str_radix("7", 10).expect("Failed to convert string to BigUint"));
    assert_eq!(lowest_prime_factor, expected_lowest_factor);
}
