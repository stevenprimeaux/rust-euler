pub fn prime_nth(n: usize) -> u64 {
    let mut count: usize = 1;
    let mut try_current: u64 = 1;

    while count < n {
        try_current += 2;
        if is_prime(try_current) {
            count += 1;
        }
    }

    try_current
}

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
    let primes_len: usize = sieve
        .iter()
        .filter(|x: &&bool| !**x)
        .collect::<Vec<&bool>>()
        .len();

    let mut primes: Vec<u64> = vec![0; primes_len];
    let mut index: usize = 1;
    primes[0] = 2;

    for (i, is_composite) in sieve.iter().enumerate().skip(1) {
        if !is_composite {
            primes[index] = ((2 * i) + 1) as u64;
            index += 1;
        }
    }

    primes
}

pub fn prime_sum(limit: usize) -> u64 {
    let sieve: Vec<bool> = prime_sieve(limit);
    let mut sum: u64 = 2;
    for (k, is_composite) in sieve.iter().enumerate().skip(1) {
        if !is_composite {
            sum += ((2 * k) + 1) as u64;
        }
    }

    sum
}

pub fn prime_fact_max(mut div_current: u64) -> u64 {
    let mut max_current: u64 = 1;

    if div_current % 2 == 0 {
        max_current = 2;
        while div_current % 2 == 0 {
            div_current /= 2;
        }
    }

    let mut try_current: u64 = 3;
    let mut limit: u64 = (div_current as f64).sqrt() as u64;
    while div_current > 1 && try_current <= limit {
        if div_current % try_current == 0 {
            max_current = try_current;
            while div_current % try_current == 0 {
                div_current /= try_current;
            }
            limit = (div_current as f64).sqrt() as u64;
        }
        try_current += 2;
    }

    if div_current == 1 {
        return max_current;
    }

    div_current
}

pub fn is_prime(n: u64) -> bool {
    match n {
        0 => false,
        1 => false,
        x => prime_fact_max(x) == x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_nth() {
        assert_eq!(prime_nth(10001), 104743);
    }

    #[test]
    fn test_prime_sieve() {
        assert_eq!(
            prime_sieve(91),
            vec![
                false, false, false, false, true, false, false, true, false, false, true, false,
                true, true, false, false, true, true, false, true, false, false, true, false, true,
                true, false, true, true, false, false, true, true, false, true, false, false, true,
                true, false, true, false, true, true, false,
            ]
        );
    }

    #[test]
    fn test_prime_array() {
        assert_eq!(prime_array(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_prime_sum() {
        assert_eq!(prime_sum(2000000), 142913828922);
    }

    #[test]
    fn test_prime_fact_max() {
        assert_eq!(prime_fact_max(600851475143), 6857);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
    }
}
