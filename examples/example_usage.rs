use num_bigint::BigUint;

use pfact::pollards_rho;

fn main() {
    let number: usize = 3838740211;
    let prime_factors = pollards_rho::get_list_of_prime_factors(BigUint::from(number));
    println!("The prime factors of {} are: {:?}", number, prime_factors);
}
