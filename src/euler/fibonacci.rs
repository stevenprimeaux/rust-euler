pub fn fib_n_even(n: usize) -> u32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 2;
    }

    4 * fib_n_even(n - 1) + fib_n_even(n - 2)
}

pub fn fib_sum_even(limit: u32) -> u32 {
    let mut index: usize = 1;
    let mut term: u32 = 2;
    let mut sum: u32 = 0;

    while term <= limit {
        sum += term;
        index += 1;
        term = fib_n_even(index);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::euler::fibonacci;

    #[test]
    fn test_fib_n_even() {
        assert_eq!(fibonacci::fib_n_even(3), 34);
    }

    #[test]
    fn test_fib_sum_even() {
        assert_eq!(fibonacci::fib_sum_even(4000000), 4613732);
    }
}
