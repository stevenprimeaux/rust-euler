use super::divisors;

/// Returns whether a number is abundant or not.
///
/// A number is called abundant if the sum of its proper divisors is greater than the number itself.
///
/// # Examples
///
/// ```
/// use rust_euler::euler::abundant::is_abundant;
///
/// assert_eq!(is_abundant(12), true);
/// assert_eq!(is_abundant(13), false);
/// assert_eq!(is_abundant(14), false);
/// ```
pub fn is_abundant(dividend: u64) -> bool {
    let sum: u64 = divisors::divs_sum_proper(dividend);

    sum > dividend
}

/// Returns sum of all numbers up to a specified value that cannot be written as the sum of two abundant numbers.
///
/// Note that all integers greater than 28123 can be written as the sum of two abundant numbers.
///
/// # Examples
///
/// ```
/// use rust_euler::euler::abundant::abund_sum_addends_notabundant;
///
/// assert_eq!(abund_sum_addends_notabundant(23), 276);
/// assert_eq!(abund_sum_addends_notabundant(24), 276);
/// assert_eq!(abund_sum_addends_notabundant(25), 301);
/// ```
/// In the example above, note that 12 is the smallest abundant number, so the smallest integer that can be written as the sum of two abundant numbers is 24.
/// As a result, the running sum increments for all values up to 23, but not from 23 to 24.
pub fn abund_sum_addends_notabundant(limit: u64) -> u64 {
    let mut abundants: Vec<u64> = vec![];
    let mut sieve: Vec<bool> = vec![false; limit as usize + 1];
    let mut sum: u64 = 0;

    for a in 12..=limit {
        if is_abundant(a) {
            abundants.push(a);
        }
    }

    for a_1 in &abundants {
        for a_2 in &abundants {
            let sum_current: u64 = a_1 + a_2;
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
        assert_eq!(is_abundant(12), true);
        assert_eq!(is_abundant(28), false);
    }

    #[test]
    fn test_abund_sum_addends_notabundant() {
        assert_eq!(abund_sum_addends_notabundant(28123), 4179871);
    }
}
