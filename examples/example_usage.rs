use std::str::FromStr;

use num_bigint::BigUint;

use pfact::pollards_rho;

fn main() {
    let number: String = "14988988966695224387".to_string();
    let prime_factors = pollards_rho::get_list_of_prime_factors(BigUint::from_str(&number).expect("Failed to convert string to BigUint"));
    println!("The prime factors of {} are: {:?}", number, prime_factors);
}
