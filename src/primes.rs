pub fn get_sieve_primes(limit: usize) -> Vec<bool> {
    let sieve_bound = (limit - 1) / 2;
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

pub fn get_array_primes(limit: usize) -> Vec<u64> {
    let sieve: Vec<bool> = get_sieve_primes(limit);
    let array_len: usize = sieve
        .iter()
        .filter(|x: &&bool| !**x)
        .collect::<Vec<&bool>>()
        .len();

    let mut array_primes: Vec<u64> = vec![0; array_len];
    array_primes[0] = 2;
    let mut array_index_current: usize = 1;
    for i in 1..sieve.len() {
        if !sieve[i] {
            array_primes[array_index_current] = ((2 * i) + 1) as u64;
            array_index_current += 1;
        }
    }

    array_primes
}

pub fn sum_primes_below(limit: usize) -> usize {
    let sieve: Vec<bool> = get_sieve_primes(limit);
    let mut sum: usize = 2;
    for k in 1..sieve.len() {
        if !sieve[k] {
            sum += (2 * k) + 1;
        }
    }

    sum
}
