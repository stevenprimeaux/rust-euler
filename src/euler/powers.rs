use crate::overflow;

/// Return the number of unique terms that can be generated
/// by a**b given all integer combinations such that
/// 2 <= a <= base_max; 2 <= b <= pow_max.
///
/// # Examples
///
/// ```
/// use rust_euler::powers::pow_count;
///
/// assert_eq!(pow_count(5, 5), 15);
/// ```
pub fn pow_count(base_max: u64, pow_max: u32) -> usize {
    let mut terms: Vec<Vec<u64>> = vec![vec![4_u64]];

    for base in 2..=base_max {
        for pow in 2..=pow_max {
            let exp_current: Vec<u64> = overflow::oflow_pow_vec(base, pow);

            if !terms.contains(&exp_current) {
                terms.push(exp_current)
            }
        }
    }

    terms.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_count() {
        assert_eq!(pow_count(100, 100), 9183);
    }
}
