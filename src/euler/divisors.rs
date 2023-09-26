pub fn divs_get_prime(dividend: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = vec![];
    let mut powers: Vec<u32> = vec![];

    let mut div_current: u64 = dividend;

    let mut power_2: u32 = 0;
    if div_current % 2 == 0 {
        divisors.push(2);

        while div_current % 2 == 0 {
            div_current /= 2;
            power_2 += 1;
        }
        powers.push(power_2);
    }

    let mut pow_current: u32 = 0;
    let mut try_current: u64 = 3;
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

    let n_factors: usize = powers.iter().map(|x: &u32| (x + 1) as usize).product();

    // println!("current: {}", dividend);

    // println!("primes:");
    // for d in &divisors {
    //     println!("{}", d);
    // }

    // println!("powers:");
    // for p in &powers {
    //     println!("{}", p);
    // }
    
    // println!("combinations: {}", n_factors);

    let mut powers_grid = vec![vec![0; n_factors]; divisors.len()];
    let mut period: usize = n_factors;
    let mut combs: Vec<u32>;
    for (i, p) in powers.iter().enumerate() {
        period = period / (*p + 1) as usize;
        
        combs = (0..=*p)
                .flat_map(|n| std::iter::repeat(n)
                .take(period))
                .cycle()
                .take(n_factors)
                .collect();

        powers_grid[i] = combs;
        
    }

    

    let mut factors = vec![1; n_factors];
    let mut factor = 1;
    for c in 0..n_factors {
        for (i, p) in divisors.iter().enumerate() {
            factor *= p.pow(powers_grid[i][c]);
        }
        factors[c] = factor;
        factor = 1;
    }

    factors.sort();

    // for (i, _) in powers_grid.iter().enumerate() {
    //     println!("divisor: {}", divisors[i]);
    //     for j in &powers_grid[i] {
    //         println!("{j}");
    //     }
    // }

    // println!("factors:");
    // for f in &factors {
    //     println!("{}", f);
    // }

    factors
}
