# Rust Prime Factorization Library

This Rust library provides various prime factorization algorithms for integers. Choose the most suitable algorithm for your needs.

## Algorithms
The following prime factorization algorithms:

1. Trial Division
2. Wheel Factorization (TODO)
3. Pollard's Rho Algorithm (TODO)
4. Algebraic Group Factorization (TODO)
5. Fermat's Factorization Method (TODO)
6. Euler's Factorization Method (TODO)
7. Special Number Field Sieve (TODO)
8. Dixon's Algorithm (TODO)
9. Continued Fraction Factorization (TODO)
10. Quadratic Sieve (TODO)
11. Rational Sieve (TODO)
12. General Number Field Sieve (TODO)
13. Shanks's Square Form Factorization (TODO)
15. Pollard's p-1 Factorization (TODO)

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
fn main() {
    let number = 1234567;
    let factors = trial_division::factorize(number);
    println!("Prime factors: {:?}", factors);
}
// Replace trial_division with the desired algorithm in the example above.
```
