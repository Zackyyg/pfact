# Rust Prime Factorization Library

This Rust library provides various prime factorization algorithms for integers. Choose the most suitable algorithm for your needs.

## Algorithms
The following prime factorization algorithms:

1. Trial Division
2. Pollard's Rho Algorithm (TODO)
3. Algebraic Group Factorization (TODO)
4. Fermat's Factorization Method (TODO)
5. Euler's Factorization Method (TODO)
6. Special Number Field Sieve (TODO)
7. Dixon's Algorithm (TODO)
8. Continued Fraction Factorization (TODO)
9. Quadratic Sieve (TODO)
10. Rational Sieve (TODO)
11. General Number Field Sieve (TODO)
12. Shanks's Square Form Factorization (TODO)
13. Pollard's p-1 Factorization (TODO)

## Usage

To use this library, add it as a dependency in your Cargo.toml file:
``` toml
[dependencies]
pfact = { path = "path/to/pfact"}
```

Then, import the desired algorithm(s) in your Rust source code:

```rust
extern crate pfact;

use pfact::trial_division;
use pfact::pollards_rho;
// ... import other algorithms as needed
```

To factorize a number using a specific algorithm:

```rust
use num_bigint::BigUint;
use pfact::trial_division;

fn main() {
    let number: BigUint = BigUint::from(1234567u32);
    let factor = trial_division::get_lowest_prime_factor(number);
    println!("Prime factors: {}", factor);
}
// Replace trial_division with the desired algorithm in the example above.
```
