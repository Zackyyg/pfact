use num_bigint::BigUint;

use pfact::trial_division::get_list_of_prime_factors;

fn main() {
    let number = 56;
    let prime_factors = get_list_of_prime_factors(BigUint::from(56u32));
    println!("The prime factors of {} are: {:?}", number, prime_factors);
}
