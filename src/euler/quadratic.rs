use crate::primes;

pub fn quad_primes_count(a: i64, b: i64) -> i64 {
    let mut n_max: i64 = 0;
    let mut prod_best: i64 = 0;

    let mut n_current: i64;
    for a in (-1 * a)..=a {
        'b: for b in (-1 * b)..=b {
            n_current = 0;
            loop {
                match primes::is_prime(quad_eval(a, b, n_current).abs().try_into().unwrap()) {
                    true => n_current += 1,
                    false => {
                        match n_current > n_max {
                            true => {
                                n_max = n_current;
                                prod_best = a * b;
                            }
                            false => (),
                        }
                        continue 'b;
                    }
                }
            }
        }
    }

    prod_best
}

pub fn quad_eval(a: i64, b: i64, n: i64) -> i64 {
    n.pow(2) + (a * n) + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_primes_count() {
        assert_eq!(quad_primes_count(1, 41), -41);
        assert_eq!(quad_primes_count(79, 1601), -126479);
        assert_eq!(quad_primes_count(1000, 1000), -59231);
    }
}
