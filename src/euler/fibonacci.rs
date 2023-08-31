pub fn fib_nth_even(n: usize) -> u32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 2;
    }

    4 * fib_nth_even(n - 1) + fib_nth_even(n - 2)
}

pub fn fib_sum_even(limit: u32) -> u32 {
    let mut index: usize = 1;
    let mut term: u32 = 2;
    let mut sum: u32 = 0;

    while term <= limit {
        sum += term;
        index += 1;
        term = fib_nth_even(index);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_nth_even() {
        assert_eq!(fib_nth_even(3), 34);
    }

    #[test]
    fn test_fib_sum_even() {
        assert_eq!(fib_sum_even(4000000), 4613732);
    }
}
