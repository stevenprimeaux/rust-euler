use super::util;

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

pub fn is_palindrome(n: u32) -> bool {
    n == util::reverse(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pal_product_3() {
        assert_eq!(pal_product_3(), 906609);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(123456), false);
        assert_eq!(is_palindrome(123321), true);
    }
}
