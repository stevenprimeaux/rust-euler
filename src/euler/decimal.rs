use crate::{divisors::Factorization, modulo, primes};

pub fn dec_period_max(limit: u64) -> u64 {
    let mut n_max: u64 = 0;
    let mut period_max: u64 = 0;

    let mut period_current: u64;
    let mut n_reduced: u64;
    for n in 1..limit {
        if is_repeating(n) {
            if primes::is_prime(n) && modulo::is_primrootmod(10, n) {
                period_current = n - 1;
            } else {
                n_reduced = n;
                while n_reduced % 2 == 0 {
                    n_reduced /= 2;
                }
                while n_reduced % 5 == 0 {
                    n_reduced /= 5;
                }

                period_current = modulo::mod_order(10, n_reduced);
            }

            if period_current > period_max {
                period_max = period_current;
                n_max = n;
            }
        }
    }

    n_max
}

pub fn is_repeating(n: u64) -> bool {
    Factorization::new(n)
        .divs_prime
        .iter()
        .any(|&x| x != 2 && x != 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_period() {
        assert_eq!(dec_period_max(1000), 983);
    }

    #[test]
    fn test_is_repeating() {
        assert_eq!(is_repeating(2), false);
        assert_eq!(is_repeating(3), true);
        assert_eq!(is_repeating(4), false);
        assert_eq!(is_repeating(5), false);
        assert_eq!(is_repeating(6), true);
    }
}
