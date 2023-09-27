use std::iter::repeat;

pub fn divs_all(dividend: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = vec![];
    let mut powers: Vec<u32> = vec![];

    let mut div_current: u64 = dividend;
    let mut try_current: u64 = 2;
    let mut pow_current: u32 = 0;

    if div_current % try_current == 0 {
        divisors.push(try_current);

        while div_current % try_current == 0 {
            div_current /= try_current;
            pow_current += 1;
        }
        powers.push(pow_current);
    }

    try_current = 3;
    pow_current = 0;

    let mut sqrt_current: u64 = (div_current as f64).sqrt() as u64;
    while div_current > 1 && try_current <= sqrt_current {
        if div_current % try_current == 0 {
            divisors.push(try_current);

            while div_current % try_current == 0 {
                div_current /= try_current;
                pow_current += 1;
            }
            powers.push(pow_current);

            sqrt_current = (div_current as f64).sqrt() as u64;
            pow_current = 0;
        }
        try_current += 2;
    }

    if div_current != 1 {
        divisors.push(div_current);
        powers.push(1);
    }

    let n_factors: u32 = powers.iter().map(|x: &u32| (x + 1)).product();
    let mut powers_grid: Vec<Vec<u32>> = vec![vec![0; n_factors as usize]; divisors.len()];

    let mut period: usize = n_factors as usize;
    let mut combos: Vec<u32>;
    for (i, p) in powers.iter().enumerate() {
        period = period / (*p + 1) as usize;

        combos = (0..=*p)
            .flat_map(|x: u32| repeat(x).take(period))
            .cycle()
            .take(n_factors as usize)
            .collect();

        powers_grid[i] = combos;
    }

    let mut factors: Vec<u64> = vec![0; n_factors as usize];
    let mut factor: u64 = 1;
    for j in 0..(n_factors as usize) {
        for (i, p) in divisors.iter().enumerate() {
            factor *= p.pow(powers_grid[i][j]);
        }
        factors[j] = factor;
        factor = 1;
    }
    factors.sort();

    factors
}

pub fn divs_count(dividend: u64) -> usize {
    divs_all(dividend).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divs_all() {
        assert_eq!(
            divs_all(220),
            vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110, 220]
        );
        assert_eq!(divs_all(284), vec![1, 2, 4, 71, 142, 284]);
    }

    #[test]
    fn test_divs_count() {
        assert_eq!(divs_count(220), 12);
        assert_eq!(divs_count(284), 6);
    }
}
