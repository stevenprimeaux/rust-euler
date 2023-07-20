fn main() {
    println!("{}", sum_divisible_by_2(1000, 3, 5));
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
