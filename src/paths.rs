pub fn path_count(dim: u128) -> u128 {
    let mut numerator: u128 = 1;
    let mut denominator: u128 = 1;

    for i in (dim + 1)..=(dim * 2) {
        numerator *= i;
    }

    for i in 2..=dim {
        denominator *= i;
    }

    numerator / denominator
}
