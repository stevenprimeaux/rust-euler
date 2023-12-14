use std::iter::zip;

use crate::divisors::Factorization;

/// Given an integer n, counts the number of positive integers below n
/// that are relatively prime to n, including 1.
///
/// Two numbers are relatively prime if their greatest common divisor is 1.
///
/// # Examples
///
/// ```
/// use rust_euler::totient::tot_count;
///
/// assert_eq!(tot_count(9), 6);    // 1, 2, 4, 5, 7, 8
/// assert_eq!(tot_count(10), 4);   // 1, 3, 7, 9
/// assert_eq!(tot_count(11), 10);  // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
/// assert_eq!(tot_count(12), 4);   // 1, 5, 7, 11
/// ```
pub fn tot_count(dividend: u64) -> u64 {
    let mut totient: u64 = 1;
    let fact: Factorization = Factorization::new(dividend);

    for (p, k) in zip(fact.divs_prime, fact.powers) {
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
