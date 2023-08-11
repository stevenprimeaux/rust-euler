pub fn reverse(mut n: u32) -> u32 {
    let mut n_rev: u32 = 0;
    while n > 0 {
        n_rev = (10 * n_rev) + (n % 10);
        n /= 10;
    }

    n_rev
}

pub fn is_palindrome(n: u32) -> bool {
    n == reverse(n)
}

pub fn pal_product_3() -> u32 {
    let mut max_current: u32 = 0;
    let mut fact_1: u32 = 990;
    let mut fact_2: u32;
    let mut dec_2: u32;

    while fact_1 >= 100 {
        if fact_1 % 11 == 0 {
            fact_2 = 999;
            dec_2 = 1;
        } else {
            fact_2 = 990;
            dec_2 = 11;
        }

        while fact_2 >= fact_1 {
            if fact_1 * fact_2 <= max_current {
                break;
            }
            if is_palindrome(fact_1 * fact_2) {
                max_current = fact_1 * fact_2;
            }
            fact_2 -= dec_2;
        }
        fact_1 -= 1;
    }

    max_current
}
