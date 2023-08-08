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
        }
        current_try += 2;
    }

    if current_dividend != 1 {
        n_factors *= 2;
    }

    n_factors
}

pub fn triangular_divisors_n(n_divisors: u64) -> u64 {
    let mut num_current: u64 = 1;
    let mut sum_current: u64 = 0;
    let mut found = false;

    while !found {
        sum_current += num_current;
        // println!(
        //     "{} {} {}",
        //     num_current,
        //     sum_current,
        //     count_divisors(sum_current)
        // );

        if count_divisors(sum_current) > n_divisors {
            found = true;
        }
        num_current += 1;
    }

    sum_current
}
