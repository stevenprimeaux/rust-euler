pub fn sum_mult(mult: u32, below: u32) -> u32 {
    let sequence_end: u32 = (below - 1) / mult;

    mult * (sequence_end * (sequence_end + 1) / 2)
}

pub fn sum_mult_2(mult_1: u32, mult_2: u32, below: u32) -> u32 {
    let sum_1: u32 = sum_mult(mult_1, below);
    let sum_2: u32 = sum_mult(mult_2, below);
    let sum_both: u32 = sum_mult(mult_1 * mult_2, below);

    sum_1 + sum_2 - sum_both
}
