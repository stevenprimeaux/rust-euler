use std::collections::HashMap;

/// Breaks a number into counts of powers of 10.
///
/// # Examples
///
/// ```
/// use rust_euler::util::counts_10;
/// use std::collections::HashMap;
///
/// assert_eq!(counts_10(123), HashMap::from([(1000, 0), (100, 1), (10, 2), (1, 3)]));
/// ```
pub fn counts_10(mut n: u32) -> HashMap<u32, usize> {
    let mut counts: HashMap<u32, usize> = HashMap::from([(1000, 0), (100, 0), (10, 0), (1, 0)]);

    if n >= 1000 {
        *counts.get_mut(&1000).unwrap() = (n as usize) / 1000;
        n %= 1000;
    }
    if n >= 100 {
        *counts.get_mut(&100).unwrap() = (n as usize) / 100;
        n %= 100;
    }
    if n >= 10 {
        *counts.get_mut(&10).unwrap() = (n as usize) / 10;
        n %= 10;
    }
    *counts.get_mut(&1).unwrap() = n as usize;

    counts
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

/// Reverses an integer.
///
/// # Examples
///
/// ```
/// use rust_euler::util::reverse;
///
/// assert_eq!(reverse(123), 321);
/// ```
///
pub fn reverse(mut n: u32) -> u32 {
    let mut n_rev: u32 = 0;
    while n > 0 {
        n_rev = (10 * n_rev) + (n % 10);
        n /= 10;
    }

    n_rev
}

pub fn sum_terms(n_terms: u32) -> u32 {
    n_terms * (n_terms + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_10() {
        assert_eq!(
            counts_10(3),
            HashMap::from([(1000, 0), (100, 0), (10, 0), (1, 3)])
        );
        assert_eq!(
            counts_10(33),
            HashMap::from([(1000, 0), (100, 0), (10, 3), (1, 3)])
        );
        assert_eq!(
            counts_10(333),
            HashMap::from([(1000, 0), (100, 3), (10, 3), (1, 3)])
        );
        assert_eq!(
            counts_10(3333),
            HashMap::from([(1000, 3), (100, 3), (10, 3), (1, 3)])
        );
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(4, 9), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(3, 6), 3);
        assert_eq!(gcd(6, 9), 3);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123456), 654321);
        assert_eq!(reverse(123321), 123321);
    }

    #[test]
    fn test_sum_terms() {
        assert_eq!(sum_terms(10), 55);
        assert_eq!(sum_terms(100), 5050);
    }
}
