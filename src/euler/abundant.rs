use crate::divisors;

/// Given a number, returns whether the sum of that number's proper divisors
/// is greater than the number itself.
///
/// # Examples
///
/// ```
/// use rust_euler::abundant::is_abundant;
///
/// assert_eq!(is_abundant(12), true);  // (1 + 2 + 3 + 4 + 6) > 12
/// assert_eq!(is_abundant(13), false); // (1) < 13
/// assert_eq!(is_abundant(14), false); // (1 + 2 + 7) < 14
/// ```
pub fn is_abundant(dividend: u64) -> bool {
    divisors::divs_proper_sum(dividend) > dividend
}

/// Returns sum of all numbers up to a specified value that cannot be written
/// as the sum of two abundant numbers.
///
/// Note that all integers greater than 28123
/// can be written as the sum of two abundant numbers.
///
/// # Examples
///
/// ```
/// use rust_euler::abundant::abund_adds_notabund_sum;
///
/// assert_eq!(abund_adds_notabund_sum(22), 253);
/// assert_eq!(abund_adds_notabund_sum(23), 276); // increment by 23
/// assert_eq!(abund_adds_notabund_sum(24), 276); // don't increment
/// assert_eq!(abund_adds_notabund_sum(25), 301); // increment by 25
/// ```
///
/// Note that 12 is the smallest abundant number,
/// so the smallest integer that can be written
/// as the sum of two abundant numbers is 24.
/// As a result, the running sum increments for all values up to 23,
/// but not from 23 to 24.
pub fn abund_adds_notabund_sum(limit: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut abundants: Vec<u64> = vec![];
    let mut sieve: Vec<bool> = vec![false; (limit as usize) + 1];

    for a in 12..=limit {
        if is_abundant(a) {
            abundants.push(a);
        }
    }

    let mut sum_current: u64;
    for a_1 in &abundants {
        for a_2 in &abundants {
            sum_current = a_1 + a_2;
            if sum_current <= limit {
                sieve[sum_current as usize] = true;
            }
        }
    }

    for (i, v) in sieve.iter().enumerate() {
        if !v {
            sum += i as u64;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abundant() {
        assert_eq!(is_abundant(24), true);
        assert_eq!(is_abundant(26), false);
        assert_eq!(is_abundant(28), false);
    }

    #[test]
    fn test_abund_sum_addends_notabundant() {
        assert_eq!(abund_adds_notabund_sum(28123), 4179871);
    }
}
