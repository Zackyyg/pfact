use num_bigint::BigUint;
use num_traits::Num;

use pfact::trial_division::{get_list_of_prime_factors};


#[test]
fn test_get_two_primes() {
    let expected_factors = vec![
        BigUint::from(13usize),
        BigUint::from(19usize),
    ];
    let prime_factors = get_list_of_prime_factors(BigUint::from_str_radix("247", 10).expect("Failed to convert string to BigUint"));
    assert_eq!(prime_factors, expected_factors);

}
