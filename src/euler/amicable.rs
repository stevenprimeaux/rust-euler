use crate::divisors;

/// Given two integers a and b, returns whether
/// a is equal to the sum of the proper divisors of b.
///
/// # Examples
///
/// ```
/// use rust_euler::amicable::is_amicable;
///
/// assert_eq!(is_amicable(220, 284), true);
/// assert_eq!(is_amicable(1184, 1210), true);
/// assert_eq!(is_amicable(2620, 2924), true);
/// ```
pub fn is_amicable(a: u64, b: u64) -> bool {
    a == divisors::divs_proper(b).iter().sum()
}

/// Returns the sum of all amicable numbers below a given value.
///
/// # Examples
///
/// ```
/// use rust_euler::amicable::ami_sum;
///
/// assert_eq!(ami_sum(300), 504);      // (220 + 284)
/// assert_eq!(ami_sum(1300), 2898);    // (1184 + 1210) + 504
/// assert_eq!(ami_sum(3000), 8442);    // (2620 + 2924) + 2898
/// ```
pub fn ami_sum(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for a in 2..limit {
        let b: u64 = divisors::divs_proper(a).iter().sum();

        if a != b && b < limit {
            if is_amicable(a, b) {
                sum += a + b;
            }
        }
    }

    sum / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_amicable() {
        assert_eq!(is_amicable(219, 283), false);
        assert_eq!(is_amicable(220, 284), true);
        assert_eq!(is_amicable(221, 285), false);
    }

    #[test]
    fn test_ami_sum() {
        assert_eq!(ami_sum(10000), 31626);
    }
}
