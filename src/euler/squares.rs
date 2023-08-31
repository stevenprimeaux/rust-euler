use crate::euler::util;

pub fn sq_sum_squares(n_terms: u32) -> u32 {
    (2 * n_terms + 1) * (n_terms + 1) * n_terms / 6
}

pub fn sq_square_sum(n_terms: u32) -> u32 {
    util::sum_terms(n_terms).pow(2)
}

pub fn sq_diff(n_terms: u32) -> u32 {
    sq_square_sum(n_terms) - sq_sum_squares(n_terms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sq_sum_squares() {
        assert_eq!(sq_sum_squares(10), 385);
    }

    #[test]
    fn test_sq_square_sum() {
        assert_eq!(sq_square_sum(10), 3025);
    }

    #[test]
    fn test_sq_diff() {
        assert_eq!(sq_diff(10), 2640);
        assert_eq!(sq_diff(100), 25164150);
    }
}
