use num_bigint::BigUint;

use trail_division::get_list_of_prime_factors;
use trail_division::get_lowest_prime_factor;


#[test]
fn test_get_list_of_prime_factors() {
    let number = 56;
    let expected_factors = vec![
        BigUint::from(2usize);
        BigUint::from(7usize);
    ];
    let prime_factors = get_list_of_prime_factors(number);
    assert_eq!(prime_factors, expected_factors);
}

#[test]
fn test_get_lowest_prime_factor() {
    let number = 56;
    let expected_lowest_factor = BigUint::from(2usize);
    let lowest_prime_factor = get_lowest_prime_factor(number);
    assert_eq!(lowest_prime_factor, expected_lowest_factor);
}
