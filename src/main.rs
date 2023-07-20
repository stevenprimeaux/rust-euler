fn main() {
    println!("{}", sum_divisible_by_2(1000, 3, 5));
    println!("{}", fibonacci_sum_even_upto(4000000));
}

fn sum_divisible_by(below: u32, mult: u32) -> u32 {
    let sequence_end: u32 = (below - 1) / mult;

    mult * (sequence_end * (sequence_end + 1) / 2)
}

fn sum_divisible_by_2(below: u32, mult_1: u32, mult_2: u32) -> u32 {
    let sum_1: u32 = sum_divisible_by(below, mult_1);
    let sum_2: u32 = sum_divisible_by(below, mult_2);
    let sum_both: u32 = sum_divisible_by(below, mult_1 * mult_2);

    sum_1 + sum_2 - sum_both
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
        current_term = fibonacci_nth_even(current_index)
    }

    current_sum
}
