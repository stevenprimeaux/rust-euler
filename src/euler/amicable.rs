use crate::divisors;

pub fn ami_sum(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for a in 2..limit {
        let b: u64 = divisors::divs_all(a)
            .iter()
            .filter(|x: &&u64| x != &&a)
            .sum();

        if a != b && b < limit {
            if a == divisors::divs_all(b)
                .iter()
                .filter(|x: &&u64| x != &&b)
                .sum()
            {
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
    fn test_ami_sum() {
        assert_eq!(ami_sum(10000), 31626);
    }
}
