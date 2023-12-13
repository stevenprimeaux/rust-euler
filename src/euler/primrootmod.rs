use crate::primes::is_prime;

use super::totient;

pub fn is_primrootmod(g: u64, n: u64) -> bool {
    let tot: u64 = totient::divs_totient(n);
    let divs_prime_tot: Vec<u64> = totient::divs_prime(tot).divisors_prime;

    let mut is_primrootmod = false;

    if is_prime(n) || ((n % 2 == 0) && is_prime(n / 2)) {
        is_primrootmod = true;
        for d in divs_prime_tot {
            let exp: u64 = tot / d;
            if exp_mod(g, exp, n) == 1 {
                return false;
            }
        }
    }

    is_primrootmod
}

pub fn dec_period(limit: u64) -> u64 {
    let mut max: u64 = 0;
    let mut max_totient: u64 = 0;
    let mut current_totient: u64;

    for n in 6..limit {
        current_totient = totient::divs_totient(n);
        if current_totient > max_totient && is_primrootmod(10, n) {
            max_totient = current_totient;
            max = n;
        }
    }

    max
}

pub fn exp_mod(base: u64, exp: u64, m: u64) -> u64 {
    let mut c: u64 = 1;
    let mut exp_current: u64 = 0;

    while exp_current < exp {
        exp_current += 1;
        c = (base * c) % m;
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_period() {
        assert_eq!(dec_period(1000), 983);
    }
}
