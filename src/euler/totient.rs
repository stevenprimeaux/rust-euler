use std::iter::zip;

pub struct Factorization {
    pub divisors_prime: Vec<u64>,
    pub powers: Vec<u32>,
}

pub fn divs_prime(dividend: u64) -> Factorization {
    let mut divisors_prime: Vec<u64> = vec![];
    let mut powers: Vec<u32> = vec![];

    let mut div_current: u64 = dividend;
    let mut try_current: u64 = 2;
    let mut pow_current: u32 = 0;

    if div_current % try_current == 0 {
        divisors_prime.push(try_current);

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
            divisors_prime.push(try_current);

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
        divisors_prime.push(div_current);
        powers.push(1);
    }

    Factorization {
        divisors_prime,
        powers,
    }
}

pub fn divs_totient(dividend: u64) -> u64 {
    let fact: Factorization = divs_prime(dividend);
    let mut totient: u64 = 1;

    for (p, k) in zip(fact.divisors_prime, fact.powers) {
        totient *= p.pow(k - 1) * (p - 1);
    }

    totient
}
