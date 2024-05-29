use super::grid;

pub fn oflow_fix(digits_vec: &mut Vec<u64>) {
    let mut is_overflow: bool = true;
    let mut digit_current: u64;

    while is_overflow {
        is_overflow = false;
        for i in 0..(digits_vec.len()) {
            digit_current = digits_vec[i];
            if digit_current >= 10 {
                if i == (digits_vec.len() - 1) {
                    digits_vec.push(0);
                }
                digits_vec[i] = digit_current % 10;
                digits_vec[i + 1] += digit_current / 10;

                is_overflow = true;
            }
        }
    }
}

pub fn oflow_factorial(n: u64) -> u64 {
    let mut digits_vec: Vec<u64> = vec![1];

    for n_current in 1..=n {
        for digit in &mut digits_vec {
            *digit *= n_current;
        }
        oflow_fix(&mut digits_vec);
    }

    digits_vec.iter().sum()
}

pub fn oflow_pow(base: u64, pow: u32) -> u64 {
    let mut digits_vec: Vec<u64> = vec![base];

    for _ in 2..=pow {
        for digit in &mut digits_vec {
            *digit *= base;
        }
        oflow_fix(&mut digits_vec);
    }

    digits_vec.iter().sum()
}

pub fn oflow_sum_rows(grid: String, n_rows: usize, n_cols: usize) -> String {
    let mut digits_vec: Vec<u64> = grid::grid_cols_sums(grid, n_rows, n_cols);
    let mut digits: String = String::from("");

    digits_vec.reverse();

    oflow_fix(&mut digits_vec);

    for d in digits_vec.iter().rev().take(10) {
        digits.push_str(&d.to_string());
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    #[test]
    fn test_oflow_fix() {
        let mut digits_vec: Vec<u64> = vec![11, 12, 13];
        oflow_fix(&mut digits_vec);
        assert_eq!(digits_vec, vec![1, 3, 4, 1]);
    }

    #[test]
    fn test_oflow_factorial() {
        assert_eq!(oflow_factorial(100), 648);
    }

    #[test]
    fn test_oflow_pow() {
        assert_eq!(oflow_pow(2, 1000), 1366);
    }

    #[test]
    fn test_oflow_sum_rows() {
        assert_eq!(oflow_sum_rows(data::data_13(), 100, 50), "5537376230");
    }
}
