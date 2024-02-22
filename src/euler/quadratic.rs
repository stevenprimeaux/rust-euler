use crate::primes;

/// Returns value of quadratic function
/// with coefficients a, b, c
/// and independent variable x.
pub fn quad_eval(a: i64, b: i64, c: i64, x: i64) -> i64 {
    (a * x.pow(2)) + (b * x) + c
}

/// Given an upper bound on coefficients b and c of a quadratic function,
/// return the product of b and c for the function that generates
/// the most consecutive prime numbers.
///
/// # Examples
///
/// ```
/// use rust_euler::quadratic::quad_primes_count;
///
/// // x^2 + x + 41, 40 primes from x = 0 to x = 39
/// assert_eq!(quad_primes_count(1, 1, 41), -41);
///
/// // x^2 - 79x + 1601, 80 primes from x = 0 to x = 79
/// assert_eq!(quad_primes_count(1, 79, 1601), -126479);
/// ```
pub fn quad_primes_count(a: i64, b: i64, c: i64) -> i64 {
    let mut n_max: i64 = 0;
    let mut prod_best: i64 = 0;

    let mut n_current: i64;
    for b in (-b)..=b {
        'c: for c in (-c)..=c {
            n_current = 0;
            loop {
                match primes::is_prime(quad_eval(a, b, c, n_current).abs().try_into().unwrap()) {
                    true => n_current += 1,
                    false => {
                        match n_current > n_max {
                            true => {
                                n_max = n_current;
                                prod_best = b * c;
                            }
                            false => (),
                        }
                        continue 'c;
                    }
                }
            }
        }
    }

    prod_best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_eval() {
        assert_eq!(quad_eval(0, 0, 0, 0), 0);
        assert_eq!(quad_eval(1, 1, 1, 1), 3);
        assert_eq!(quad_eval(2, 2, 2, 2), 14);
    }

    #[test]
    fn test_quad_primes_count() {
        assert_eq!(quad_primes_count(1, 1000, 1000), -59231);
    }
}
