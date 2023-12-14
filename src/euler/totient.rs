use std::iter::zip;

use crate::divisors;

pub fn tot_count(dividend: u64) -> u64 {
    let fact: divisors::Factorization = divisors::divs_fact(dividend);
    let mut totient: u64 = 1;

    for (p, k) in zip(fact.divisors_prime, fact.powers) {
        totient *= p.pow(k - 1) * (p - 1);
    }

    totient
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tot_count() {
        assert_eq!(tot_count(2), 1);
        assert_eq!(tot_count(3), 2);
        assert_eq!(tot_count(4), 2);
        assert_eq!(tot_count(5), 4);
        assert_eq!(tot_count(6), 2);
    }
}
