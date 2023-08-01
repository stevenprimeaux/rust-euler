fn main() {
    println!("1: {}", sum_divisible_by_2(1000, 3, 5));
    println!("2: {}", fibonacci_sum_even_upto(4000000));
    println!("3: {}", largest_prime_factor(600851475143));
    println!("4: {}", largest_palindrome_product_3());
    println!("5: {}", smallest_multiple(20));
    println!("6: {}", difference_sum_squares(100));
    println!("7: {}", get_nth_prime(10001));
    println!("8: {}", largest_product(data_8()));
    println!("9: {}", pythagorean_triple(1000));
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

fn get_nth_prime(n: u64) -> u64 {
    let mut n_primes_current: u64 = 1;
    let mut current_try: u64 = 1;

    while n_primes_current < n {
        current_try += 2;
        if is_prime(current_try) {
            n_primes_current += 1;
        }
    }

    current_try
}

fn data_8() -> String {
    String::from(
        "
        73167176531330624919225119674426574742355349194934
        96983520312774506326239578318016984801869478851843
        85861560789112949495459501737958331952853208805511
        12540698747158523863050715693290963295227443043557
        66896648950445244523161731856403098711121722383113
        62229893423380308135336276614282806444486645238749
        30358907296290491560440772390713810515859307960866
        70172427121883998797908792274921901699720888093776
        65727333001053367881220235421809751254540594752243
        52584907711670556013604839586446706324415722155397
        53697817977846174064955149290862569321978468622482
        83972241375657056057490261407972968652414535100474
        82166370484403199890008895243450658541227588666881
        16427171479924442928230863465674813919123162824586
        17866458359124566529476545682848912883142607690042
        24219022671055626321111109370544217506941658960408
        07198403850962455444362981230987879927244284909188
        84580156166097919133875499200524063689912560717606
        05886116467109405077541002256983155200055935729725
        71636269561882670428252483600823257530420752963450
        ",
    )
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
    let s_half: u64 = s / 2;
    let m_limit: u64 = ((s as f64).sqrt().ceil() - 1.0) as u64;

    let mut k: u64;
    let mut s_div_m: u64;

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;

    'outer: for m in 2..m_limit {
        if s_half % m == 0 {
            s_div_m = s_half / m;
            while s_div_m % 2 == 0 {
                s_div_m /= 2;
            }

            if m % 2 == 1 {
                k = m + 2;
            } else {
                k = m + 1;
            }

            while (k < 2 * m) & (k <= s_div_m) {
                if (s_div_m % k == 0) & (gcd(k, m) == 1) {
                    let d = s_half / (k * m);
                    let n = k - m;

                    a = d * (m * m - n * n);
                    b = 2 * d * m * n;
                    c = d * (m * m + n * n);

                    break 'outer;
                }
                k += 2;
            }
        }
    }
    a * b * c
}
