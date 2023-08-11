pub fn mult_sum(mult: u32, limit: u32) -> u32 {
    let sequence_end: u32 = (limit - 1) / mult;

    mult * (sequence_end * (sequence_end + 1) / 2)
}

pub fn mult_sum_2(mult_1: u32, mult_2: u32, limit: u32) -> u32 {
    let sum_1: u32 = mult_sum(mult_1, limit);
    let sum_2: u32 = mult_sum(mult_2, limit);
    let sum_both: u32 = mult_sum(mult_1 * mult_2, limit);

    sum_1 + sum_2 - sum_both
}
