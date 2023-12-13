use crate::euler::{modulo, totient};

pub fn dec_period_max(limit: u64) -> u64 {
    let mut n_max: u64 = 0;
    let mut period_max: u64 = 0;
    let mut totient: u64;

    for n in 1..limit {
        totient = totient::divs_totient(n);
        if totient > period_max && modulo::is_primrootmod(10, n) {
            period_max = totient;
            n_max = n;
        }
    }

    n_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_period() {
        assert_eq!(dec_period_max(1000), 983);
    }
}
