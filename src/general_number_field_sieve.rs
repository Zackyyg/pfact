use num_bigint::BigUint;


pub fn get_list_of_prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut prime_list: Vec<BigUint> = Vec::new();

    prime_list
}

fn find_primes(limit: usize) -> Vec<usize> {

    let mut prime_list: Vec<usize> = Vec::new();

    if limit > 2 {
        prime_list.push(2);
    }

    if limit > 3 {
        prime_list.push(3);
    }

    let mut sieve: Vec<bool> = vec![false; &limit + 1];

    let mut x: usize = 1;
    while x * x <= limit {

        let mut y: usize = 1;
        while y * y <= limit {

            let mut n: usize = (4 * &x * &x) + (&y * &y);
            if n <= limit && (n % 12 == 1 || n % 12 == 5){
                sieve[n] ^= true;
            }

            n = (3 * &x * &x) + (&y * &y);
            if n <= limit && n % 12 == 7 {
                sieve[n] ^= true;
            }

            let n = (3 * x * x).checked_sub(y * y);
            if (x > y) && n.is_some(){
                let n = n.unwrap();
                if n <= limit && n % 12 == 11{
                    sieve[n] ^= true;
                }
            }

            y += 1;
        }
        x += 1;
    }

    let mut r: usize = 1;
    while r * r <= limit {
        if sieve[r] {
            let mut i: usize = &r * &r;
            while i <= limit {
                sieve[i] = false;
                i = &i + &r * &r;
            }
        }
        r += 1;
    }

    let mut i: usize = 1;
    while i <= limit {
        if sieve[i] {
            prime_list.push(i.clone());
        }
        i += 1;
    }
    prime_list
}
