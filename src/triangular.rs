use crate::primes;

pub fn tri_sum(n_div: u64) -> u64 {
    let mut n_div_current: u64 = 0;

    let mut n: u64 = 3;
    let mut n_1: u64;

    let mut d_n: u64 = 2;
    let mut d_n_1: u64;
    let mut exp: u64;

    let primes: Vec<u64> = primes::prime_array(65000);

    while n_div_current <= n_div {
        n += 1;
        n_1 = n;
        if n_1 % 2 == 0 {
            n_1 /= 2
        }
        d_n_1 = 1;
        for i in 0..(primes.len() - 1) {
            if primes[i] * primes[i] > n_1 {
                d_n_1 *= 2;
                break;
            }

            exp = 1;
            while n_1 % primes[i] == 0 {
                exp += 1;
                n_1 /= primes[i];
            }
            if exp > 1 {
                d_n_1 *= exp;
            }
            if n_1 == 1 {
                break;
            }
        }
        n_div_current = d_n * d_n_1;
        d_n = d_n_1;
    }

    n * (n - 1) / 2
}

// pub fn count_divisors(mut current_dividend: u64) -> u64 {
//     let mut n_factors: u64 = 1;

//     if current_dividend % 2 == 0 {
//         let mut power_2: u64 = 0;
//         while current_dividend % 2 == 0 {
//             current_dividend /= 2;
//             power_2 += 1;
//         }
//         n_factors *= power_2 + 1;
//     }

//     let mut power_current: u64 = 0;
//     let mut current_try: u64 = 3;
//     let mut current_sqrt: u64 = (current_dividend as f64).sqrt() as u64;
//     while current_dividend > 1 && current_try <= current_sqrt {
//         if current_dividend % current_try == 0 {
//             while current_dividend % current_try == 0 {
//                 current_dividend /= current_try;
//                 power_current += 1;
//             }
//             current_sqrt = (current_dividend as f64).sqrt() as u64;

//             n_factors *= power_current + 1;
//             power_current = 0;
//         }
//         current_try += 2;
//     }

//     if current_dividend != 1 {
//         n_factors *= 2;
//     }

//     n_factors
// }

// pub fn triangular_divisors_n(n_divisors: u64) -> u64 {
//     let mut num_current: u64 = 1;
//     let mut sum_current: u64 = 0;
//     let mut found: bool = false;

//     while !found {
//         sum_current += num_current;

//         if count_divisors(sum_current) > n_divisors {
//             found = true;
//         }
//         num_current += 1;
//     }

//     sum_current
// }
