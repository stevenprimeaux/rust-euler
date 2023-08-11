use crate::primes;

pub fn mult_sum(mult: u32, limit: u32) -> u32 {
    let sequence_end: u32 = (limit - 1) / mult;

    mult * (sequence_end * (sequence_end + 1) / 2)
}

pub fn mult_sum_2(mult_1: u32, mult_2: u32, limit: u32) -> u32 {
    let sum_1: u32 = mult_sum(mult_1, limit);
    let sum_2: u32 = mult_sum(mult_2, limit);
    let sum_both: u32 = mult_sum(mult_1 * mult_2, limit);

    sum_1 + sum_2 - sum_both
}

pub fn mult_smallest(k: u64) -> u64 {
    let primes: Vec<u64> = primes::prime_array(k as usize);
    let mut primes_powers: Vec<u32> = vec![1; primes.len()];
    let mut mult: u64 = 1;

    let k: f64 = k as f64;
    let log_k: f64 = k.log(1_f64.exp());
    let limit: u64 = k.sqrt().floor() as u64;

    let mut log_p: f64;
    for i in 0..primes.len() {
        if primes[i] <= limit {
            log_p = (primes[i] as f64).log(1_f64.exp());
            primes_powers[i] = (log_k / log_p).floor() as u32;
        }
        mult *= primes[i].pow(primes_powers[i]);
    }

    mult
}
