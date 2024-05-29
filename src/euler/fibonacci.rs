use super::overflow::oflow_fix;

pub fn fib_nth_even(n: usize) -> u32 {
    if n == 0 {
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

/// Given a number of digits, returns the index of the first term in the
/// Fibonacci sequence containing that many digits.
///
/// For example, assuming the first two terms in the sequence are 1 and 1,
/// the first term with two digits occurs at index 7 (13),
/// and the first term with three digits occurs at index 12 (144).
///
/// # Examples
///
/// ```
/// use rust_euler::fibonacci::fib_oflow;
///
/// assert_eq!(fib_oflow(1), 1);
/// assert_eq!(fib_oflow(2), 7);
/// assert_eq!(fib_oflow(3), 12);
/// ```
pub fn fib_oflow(digits: usize) -> usize {
    let mut a: Vec<u64> = vec![0];
    let mut b: Vec<u64> = vec![1];
    let mut c: Vec<u64>;
    let mut digits_current: usize = 1;
    let mut index: usize = 1;

    while digits_current < digits {
        while a.len() < b.len() {
            a.push(0);
        }
        c = vec![0; b.len()];

        for (i, (a_val, b_val)) in a.iter().zip(&b).enumerate() {
            c[i] = a_val + b_val;
        }
        a.clone_from(&b);
        b.clone_from(&c);

        oflow_fix(&mut a);
        oflow_fix(&mut b);

        digits_current = b.len();
        index += 1;
    }

    index
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

    #[test]
    fn test_fib_oflow() {
        assert_eq!(fib_oflow(1000), 4782);
    }
}
