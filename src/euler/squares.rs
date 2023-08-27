use crate::util;

pub fn sq_diff(n_terms: u32) -> u32 {
    util::sum_terms(n_terms).pow(2) - util::sum_squares(n_terms)
}
