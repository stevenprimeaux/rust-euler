/// Given side length of square grid, sum diagonal values in Ulam spiral.
///
/// # Examples
///
/// ```
/// use rust_euler::spiral::spiral_diag_sum;
///
/// assert_eq!(spiral_diag_sum(1), 1);
/// assert_eq!(spiral_diag_sum(3), 25);
/// assert_eq!(spiral_diag_sum(5), 101);
/// ```
pub fn spiral_diag_sum(len_side_max: u64) -> u64 {
    let mut sum = 1;

    for len_side in (3..=len_side_max).step_by(2) {
        sum += (4 * len_side.pow(2)) - (6 * len_side) + 6
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_diag_sum() {
        assert_eq!(spiral_diag_sum(1001), 669171001);
    }
}
