use crate::euler::primes;

use super::totient;

pub fn is_primrootmod(g: u64, n: u64) -> bool {
    let tot: u64 = totient::divs_totient(n);
    let divs_prime_tot: Vec<u64> = totient::divs_prime(tot).divisors_prime;

    let mut is_primrootmod = false;

    if primes::is_prime(n) || ((n % 2 == 0) && primes::is_prime(n / 2)) {
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

pub fn exp_mod(base: u64, exp: u64, m: u64) -> u64 {
    let mut c: u64 = 1;
    let mut exp_current: u64 = 0;

    while exp_current < exp {
        exp_current += 1;
        c = (base * c) % m;
    }

    c
}
