pub fn fib_nth_even(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 2;
    }

    4 * fib_nth_even(n - 1) + fib_nth_even(n - 2)
}

pub fn fib_sum_even(upto: u32) -> u32 {
    let mut current_index: u32 = 1;
    let mut current_term: u32 = 2;
    let mut current_sum: u32 = 0;
    while current_term <= upto {
        current_sum += current_term;
        current_index += 1;
        current_term = fib_nth_even(current_index);
    }

    current_sum
}
