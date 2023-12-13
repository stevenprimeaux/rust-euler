use crate::euler::{primrootmod, totient};

pub fn dec_period<T: Into<u64>>(limit: T) -> u64 {
    let mut n_max: u64 = 0;
    let mut period_max: u64 = 0;
    let mut totient: u64;

    for n in 1..limit.into() {
        totient = totient::divs_totient(n);
        if totient > period_max && primrootmod::is_primrootmod(10, n) {
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
        assert_eq!(dec_period(1000u16), 983);
    }
}
