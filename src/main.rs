mod data;

mod fibonacci;
mod grid;
mod multiples;
mod palindrome;
mod primes;
mod triangular;

fn main() {
    println!("01: {}", multiples::mult_sum_2(3, 5, 1000));
    println!("02: {}", fibonacci::fib_sum_even(4000000));
    println!("03: {}", primes::prime_fact_max(600851475143));
    println!("04: {}", palindrome::pal_product_3());
    println!("05: {}", multiples::mult_smallest(20));
    println!("06: {}", difference_sum_squares(100));
    println!("07: {}", get_nth_prime(10001));
    println!("08: {}", largest_product(data::data_08()));
    println!("09: {}", pythagorean_triple(1000));
    println!("10: {}", primes::prime_sum(2000000));
    println!(
        "11: {}",
        grid::largest_product_grid(data::data_11(), 20, 20)
    );
    println!("12: {}", triangular::tri_sum_n_divisors(500));
    println!("13: {}", grid::grid_sum_rows(data::data_13()));
}

fn difference_sum_squares(n_terms: u32) -> u32 {
    let sum_terms: u32 = n_terms * (n_terms + 1) / 2;
    let sum_squares: u32 = (2 * n_terms + 1) * (n_terms + 1) * n_terms / 6;

    sum_terms.pow(2) - sum_squares
}

fn get_nth_prime(n: u64) -> u64 {
    let mut n_primes_current: u64 = 1;
    let mut current_try: u64 = 1;

    while n_primes_current < n {
        current_try += 2;
        if primes::is_prime(current_try) {
            n_primes_current += 1;
        }
    }

    current_try
}

fn largest_product(grid: String) -> u64 {
    grid.split("")
        .filter_map(|s: &str| s.parse().ok())
        .collect::<Vec<u64>>()
        .windows(13)
        .map(|x: &[u64]| x.iter().product())
        .max()
        .unwrap()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn pythagorean_triple(s: u64) -> u64 {
    let s_2: u64 = s / 2;
    let mut s_m: u64;

    let mut k: u64;
    let m_limit: u64 = ((s as f64).sqrt().ceil() - 1.0) as u64;

    let d: u64;
    let n: u64;

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;

    'outer: for m in 2..m_limit {
        if s_2 % m == 0 {
            s_m = s_2 / m;
            while s_m % 2 == 0 {
                s_m /= 2;
            }

            if m % 2 == 1 {
                k = m + 2;
            } else {
                k = m + 1;
            }

            while (k < 2 * m) & (k <= s_m) {
                if (s_m % k == 0) & (gcd(k, m) == 1) {
                    d = s_2 / (k * m);
                    n = k - m;

                    a = d * (m * m - n * n);
                    b = d * (2 * m * n);
                    c = d * (m * m + n * n);

                    break 'outer;
                }
                k += 2;
            }
        }
    }
    a * b * c
}
