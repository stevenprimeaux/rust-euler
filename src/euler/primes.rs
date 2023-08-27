pub fn prime_sieve(limit: usize) -> Vec<bool> {
    let sieve_bound: usize = (limit - 1) / 2;
    let mut sieve: Vec<bool> = vec![false; sieve_bound];

    let limit_cross: usize = (((limit as f64).sqrt().floor() - 1.0) / 2.0) as usize;
    for i in 1..limit_cross {
        if !sieve[i] {
            for j in ((2 * i * (i + 1))..sieve_bound).step_by((2 * i) + 1) {
                sieve[j] = true;
            }
        }
    }

    sieve
}

pub fn prime_array(limit: usize) -> Vec<u64> {
    let sieve: Vec<bool> = prime_sieve(limit);
    let array_len: usize = sieve
        .iter()
        .filter(|x: &&bool| !**x)
        .collect::<Vec<&bool>>()
        .len();

    let mut array_primes: Vec<u64> = vec![0; array_len];
    let mut array_index_current: usize = 1;
    array_primes[0] = 2;

    for i in 1..sieve.len() {
        if !sieve[i] {
            array_primes[array_index_current] = ((2 * i) + 1) as u64;
            array_index_current += 1;
        }
    }

    array_primes
}

pub fn prime_nth(n: usize) -> u64 {
    let mut n_current: usize = 1;
    let mut try_current: u64 = 1;

    while n_current < n {
        try_current += 2;
        if is_prime(try_current) {
            n_current += 1;
        }
    }

    try_current
}

pub fn prime_sum(limit: usize) -> u64 {
    let sieve: Vec<bool> = prime_sieve(limit);
    let mut sum: u64 = 2;
    for k in 1..sieve.len() {
        if !sieve[k] {
            sum += ((2 * k) + 1) as u64;
        }
    }

    sum
}

pub fn prime_fact_max(mut current_dividend: u64) -> u64 {
    let mut current_largest: u64 = 1;

    if current_dividend % 2 == 0 {
        current_largest = 2;
        while current_dividend % 2 == 0 {
            current_dividend /= 2;
        }
    }

    let mut current_try: u64 = 3;
    let mut current_sqrt: u64 = (current_dividend as f64).sqrt() as u64;
    while current_dividend > 1 && current_try <= current_sqrt {
        if current_dividend % current_try == 0 {
            current_largest = current_try;
            while current_dividend % current_try == 0 {
                current_dividend /= current_try;
            }
            current_sqrt = (current_dividend as f64).sqrt() as u64;
        }
        current_try += 2;
    }

    if current_dividend == 1 {
        return current_largest;
    }

    current_dividend
}

pub fn is_prime(n: u64) -> bool {
    prime_fact_max(n) == n
}
