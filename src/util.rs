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
