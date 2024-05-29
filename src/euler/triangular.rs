use crate::{divisors, primes};

pub fn tri_sum(n_div: u64) -> u64 {
    let mut n_div_current: u64 = 0;

    let mut n: u64 = 3;
    let mut n_1: u64;

    let mut d_n: u64 = 2;
    let mut d_n_1: u64;
    let mut exp: u64;

    let primes: Vec<u64> = primes::prime_array(65000);

    while n_div_current <= n_div {
        n += 1;
        n_1 = n;
        if n_1 % 2 == 0 {
            n_1 /= 2
        }
        d_n_1 = 1;
        for p in primes.iter().take(primes.len() - 1) {
            if p * p > n_1 {
                d_n_1 *= 2;
                break;
            }

            exp = 1;
            while n_1 % p == 0 {
                exp += 1;
                n_1 /= p;
            }
            if exp > 1 {
                d_n_1 *= exp;
            }
            if n_1 == 1 {
                break;
            }
        }
        n_div_current = d_n * d_n_1;
        d_n = d_n_1;
    }

    n * (n - 1) / 2
}

pub fn tri_sum_brute(n_div: usize) -> u64 {
    let mut num_current: u64 = 1;
    let mut sum_current: u64 = 0;
    let mut is_found: bool = false;

    while !is_found {
        sum_current += num_current;

        if divisors::divs_all_count(sum_current) > n_div {
            is_found = true;
        }
        num_current += 1;
    }

    sum_current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tri_sum() {
        assert_eq!(tri_sum(500), 76576500);
    }

    #[test]
    fn test_tri_sum_brute() {
        assert_eq!(tri_sum_brute(500), 76576500);
    }
}
