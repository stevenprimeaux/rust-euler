use crate::{divisors::Factorization, totient, util};

pub fn mod_exp(base: u64, exp: u64, m: u64) -> u64 {
    let mut result: u64 = 1;
    let mut exp_current: u64 = 0;

    while exp_current < exp {
        exp_current += 1;
        result = (base * result) % m;
    }

    result
}

pub fn mod_order(base: u64, n: u64) -> u64 {
    let mut order: u64 = 0;
    let mut m: u64 = 0;

    if util::gcd(base, n) == 1 {
        while m != 1 {
            order += 1;
            m = mod_exp(base, order, n);
        }
    }

    order
}

pub fn is_primrootmod(base: u64, n: u64) -> bool {
    let mut is_primrootmod: bool = false;
    let tot: u64 = totient::tot_count(n);
    let divs_prime: Vec<u64> = Factorization::new(tot).divs_prime;

    if n > base {
        is_primrootmod = true;
        for d in divs_prime {
            let exp: u64 = tot / d;
            if mod_exp(base, exp, n) == 1 {
                return false;
            }
        }
    }

    is_primrootmod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        assert_eq!(mod_exp(10, 10, 11), 1);
        assert_eq!(mod_exp(10, 12, 13), 1);
        assert_eq!(mod_exp(10, 16, 17), 1);
    }

    #[test]
    fn test_mod_order() {
        assert_eq!(mod_order(10, 11), 2);
        assert_eq!(mod_order(10, 13), 6);
        assert_eq!(mod_order(10, 17), 16);
    }

    #[test]
    fn test_is_primmodroot() {
        assert_eq!(is_primrootmod(10, 11), false);
        assert_eq!(is_primrootmod(10, 13), false);
        assert_eq!(is_primrootmod(10, 17), true);
    }
}
