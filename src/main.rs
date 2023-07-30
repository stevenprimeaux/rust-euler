fn main() {
    println!("1: {}", sum_divisible_by_2(1000, 3, 5));
    println!("2: {}", fibonacci_sum_even_upto(4000000));
    println!("3: {}", largest_prime_factor(600851475143));
    println!("4: {}", largest_palindrome_product_3());
    println!("5: {}", smallest_multiple(20));
    println!("6: {}", difference_sum_squares(100));
}

fn sum_divisible_by(below: u32, mult: u32) -> u32 {
    let sequence_end: u32 = (below - 1) / mult;

    mult * (sequence_end * (sequence_end + 1) / 2)
}

fn sum_divisible_by_2(below: u32, mult_a: u32, mult_b: u32) -> u32 {
    let sum_a: u32 = sum_divisible_by(below, mult_a);
    let sum_b: u32 = sum_divisible_by(below, mult_b);
    let sum_both: u32 = sum_divisible_by(below, mult_a * mult_b);

    sum_a + sum_b - sum_both
}

fn fibonacci_nth_even(index: u32) -> u32 {
    if index <= 0 {
        return 0;
    }
    if index == 1 {
        return 2;
    }

    4 * fibonacci_nth_even(index - 1) + fibonacci_nth_even(index - 2)
}

fn fibonacci_sum_even_upto(upto: u32) -> u32 {
    let mut current_index: u32 = 1;
    let mut current_term: u32 = 2;
    let mut current_sum: u32 = 0;
    while current_term <= upto {
        current_sum += current_term;
        current_index += 1;
        current_term = fibonacci_nth_even(current_index);
    }

    current_sum
}

fn largest_prime_factor(mut current_dividend: u64) -> u64 {
    let mut current_largest: u64 = 1;

    if current_dividend % 2 == 0 {
        current_largest = 2;
        while current_dividend % 2 == 0 {
            current_dividend /= 2;
        }
    }

    let mut current_try: u64 = 3;
    let mut current_sqrt: u64 = (current_dividend as f64).sqrt() as u64;
    while current_dividend > 1 && current_try <= current_sqrt {
        if current_dividend % current_try == 0 {
            current_largest = current_try;
            while current_dividend % current_try == 0 {
                current_dividend /= current_try;
            }
            current_sqrt = (current_dividend as f64).sqrt() as u64;
        }
        current_try += 2;
    }

    if current_dividend == 1 {
        return current_largest;
    }

    current_dividend
}

fn reverse(mut n_forward: u32) -> u32 {
    let mut n_backward: u32 = 0;
    while n_forward > 0 {
        n_backward = (10 * n_backward) + (n_forward % 10);
        n_forward /= 10;
    }

    n_backward
}

fn is_palindrome(n: u32) -> bool {
    n == reverse(n)
}

fn largest_palindrome_product_3() -> u32 {
    let mut current_largest: u32 = 0;
    let mut factor_a: u32 = 990;
    let mut factor_b: u32;
    let mut decrement_b: u32;

    while factor_a >= 100 {
        if factor_a % 11 == 0 {
            factor_b = 999;
            decrement_b = 1;
        } else {
            factor_b = 990;
            decrement_b = 11;
        }

        while factor_b >= factor_a {
            if factor_a * factor_b <= current_largest {
                break;
            }
            if is_palindrome(factor_a * factor_b) {
                current_largest = factor_a * factor_b;
            }
            factor_b -= decrement_b;
        }
        factor_a -= 1;
    }

    current_largest
}

fn is_prime(n: u64) -> bool {
    largest_prime_factor(n) == n
}

fn get_primes_upto(k: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![2];
    let mut current_try: u64 = 3;
    while current_try <= k {
        if is_prime(current_try) {
            primes.push(current_try);
        }
        current_try += 2;
    }

    primes
}

fn smallest_multiple(k: u64) -> u64 {
    let primes: Vec<u64> = get_primes_upto(k);
    let mut prime_powers: Vec<u32> = vec![1; primes.len()];
    let mut mult: u64 = 1;

    let k: f64 = k as f64;
    let log_k: f64 = k.log(1_f64.exp());
    let limit: u64 = k.sqrt().floor() as u64;

    let mut log_p: f64;
    for i in 0..primes.len() {
        if primes[i] <= limit {
            log_p = (primes[i] as f64).log(1_f64.exp());
            prime_powers[i] = (log_k / log_p).floor() as u32;
        }
        mult *= primes[i].pow(prime_powers[i]);
    }

    mult
}

fn difference_sum_squares(n_terms: u32) -> u32 {
    let sum_terms: u32 = n_terms * (n_terms + 1) / 2;
    let sum_squares: u32 = (2 * n_terms + 1) * (n_terms + 1) * n_terms / 6;

    sum_terms.pow(2) - sum_squares
}
