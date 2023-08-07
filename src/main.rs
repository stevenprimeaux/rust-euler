fn main() {
    println!("01: {}", sum_divisible_by_2(1000, 3, 5));
    println!("02: {}", fibonacci_sum_even_upto(4000000));
    println!("03: {}", largest_prime_factor(600851475143));
    println!("04: {}", largest_palindrome_product_3());
    println!("05: {}", smallest_multiple(20));
    println!("06: {}", difference_sum_squares(100));
    println!("07: {}", get_nth_prime(10001));
    println!("08: {}", largest_product(data_08()));
    println!("09: {}", pythagorean_triple(1000));
    println!("10: {}", sum_primes_below(2000000));
    println!("11: {}", largest_product_grid(data_11(), 20, 20));
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

fn sum_primes_below(limit: usize) -> usize {
    let sieve_bound = (limit - 1) / 2;
    let mut sieve: Vec<bool> = vec![false; sieve_bound];

    let limit_cross: usize = (((limit as f64).sqrt().floor() - 1.0) / 2.0) as usize;
    for i in 1..limit_cross {
        if !sieve[i] {
            for j in ((2 * i * (i + 1))..sieve_bound).step_by((2 * i) + 1) {
                sieve[j] = true;
            }
        }
    }

    let mut sum: usize = 2;
    for k in 1..sieve_bound {
        if !sieve[k] {
            sum += (2 * k) + 1;
        }
    }

    sum
}

fn make_2d_row(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array: Vec<Vec<u64>> = vec![vec![0; n_2]; n_1];
    let mut i_1: usize;
    let mut i_2: usize;
    for i in 0..num_vec.len() {
        i_1 = i / n_1;
        i_2 = i - (i_1 * 20);
        num_array[i_1][i_2] = num_vec[i];
    }

    num_array
}

fn make_2d_col(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array: Vec<Vec<u64>> = vec![vec![0; n_1]; n_2];
    let mut i_1: usize;
    let mut i_2: usize;
    for i in 0..num_vec.len() {
        i_1 = i / n_1;
        i_2 = i - (i_1 * 20);
        num_array[i_2][i_1] = num_vec[i];
    }

    num_array
}

fn make_2d_diag_neg(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_1];
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_2];

    for i in 0..n_1 {
        let mut current_diag: Vec<u64> = vec![];
        for j in ((i * n_1)..(num_vec.len())).step_by(n_2 + 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    for i in 0..n_2 {
        let mut current_diag: Vec<u64> = vec![];
        for j in (i..(num_vec.len() - (i * n_2))).step_by(n_2 + 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_upper[i] = current_diag;
    }

    let mut num_array_full: Vec<Vec<u64>> = num_array_lower;
    num_array_full.append(&mut num_array_upper);

    num_array_full
}

fn make_2d_diag_pos(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_1];
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_2];

    for i in 0..n_1 {
        let mut current_diag: Vec<u64> = vec![];
        for j in (i..(((i + 1) * n_1) - 1)).step_by(n_2 - 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_upper[i] = current_diag;
    }

    for i in 0..n_2 {
        let mut current_diag: Vec<u64> = vec![];
        for j in ((n_2 * i + (n_2 - 1))..(num_vec.len())).step_by(n_2 - 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    let mut num_array_full: Vec<Vec<u64>> = num_array_upper;
    num_array_full.append(&mut num_array_lower);

    num_array_full
}

fn largest_product_array(num_array: Vec<Vec<u64>>) -> u64 {
    let mut max_product: u64 = 0;
    let mut max_product_current: u64;
    for i in 0..num_array.len() {
        if num_array[i].len() >= 4 {
            max_product_current = num_array[i]
                .windows(4)
                .map(|x: &[u64]| x.iter().product())
                .max()
                .unwrap();

            if max_product_current > max_product {
                max_product = max_product_current;
            }
        }
    }

    max_product
}

fn get_max_row(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_row(&num_vec, n_1, n_2))
}

fn get_max_col(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_col(&num_vec, n_1, n_2))
}

fn get_max_diag_neg(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_diag_neg(&num_vec, n_1, n_2))
}

fn get_max_diag_pos(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_diag_pos(&num_vec, n_1, n_2))
}

fn largest_product_grid(grid: String, n_1: usize, n_2: usize) -> u64 {
    let num_vec: Vec<u64> = grid
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let maxes: [u64; 4] = [
        get_max_row(&num_vec, n_1, n_2),
        get_max_col(&num_vec, n_1, n_2),
        get_max_diag_neg(&num_vec, n_1, n_2),
        get_max_diag_pos(&num_vec, n_1, n_2),
    ];

    *maxes.iter().max().unwrap()
}

// input data

fn data_08() -> String {
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

fn data_11() -> String {
    String::from(
        "
        08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
        ",
    )
}
