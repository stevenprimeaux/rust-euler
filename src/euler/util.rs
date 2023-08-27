pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn reverse(mut n: u32) -> u32 {
    let mut n_rev: u32 = 0;
    while n > 0 {
        n_rev = (10 * n_rev) + (n % 10);
        n /= 10;
    }

    n_rev
}

pub fn sum_squares(n_terms: u32) -> u32 {
    (2 * n_terms + 1) * (n_terms + 1) * n_terms / 6
}

pub fn sum_terms(n_terms: u32) -> u32 {
    n_terms * (n_terms + 1) / 2
}

#[allow(dead_code)]
pub fn count_divisors(mut current_dividend: u64) -> u64 {
    let mut n_factors: u64 = 1;

    if current_dividend % 2 == 0 {
        let mut power_2: u64 = 0;
        while current_dividend % 2 == 0 {
            current_dividend /= 2;
            power_2 += 1;
        }
        n_factors *= power_2 + 1;
    }

    let mut power_current: u64 = 0;
    let mut current_try: u64 = 3;
    let mut current_sqrt: u64 = (current_dividend as f64).sqrt() as u64;
    while current_dividend > 1 && current_try <= current_sqrt {
        if current_dividend % current_try == 0 {
            while current_dividend % current_try == 0 {
                current_dividend /= current_try;
                power_current += 1;
            }
            current_sqrt = (current_dividend as f64).sqrt() as u64;

            n_factors *= power_current + 1;
            power_current = 0;
        }
        current_try += 2;
    }

    if current_dividend != 1 {
        n_factors *= 2;
    }

    n_factors
}
