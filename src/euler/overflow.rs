pub fn oflow_fix(digits_vec: &mut Vec<u32>) {
    let mut overflow: bool = true;
    let mut digit_current: u32;

    while overflow == true {
        overflow = false;
        for i in 0..(digits_vec.len()) {
            digit_current = digits_vec[i];
            if digit_current >= 10 {
                if i == (digits_vec.len() - 1) {
                    digits_vec.push(0);
                }
                digits_vec[i] = digit_current % 10;
                digits_vec[i + 1] += digit_current / 10;

                overflow = true;
            }
        }
    }
}

pub fn oflow_factorial(n: u32) -> u32 {
    let mut digits_vec: Vec<u32> = vec![1];

    for n_current in 1..=n {
        for i in 0..digits_vec.len() {
            digits_vec[i] *= n_current;
        }

        oflow_fix(&mut digits_vec);
    }

    digits_vec.iter().sum()
}

pub fn oflow_pow(base: u32, pow: u32) -> u32 {
    let mut digits_vec: Vec<u32> = vec![base];

    for _ in 2..=pow {
        for i in 0..digits_vec.len() {
            digits_vec[i] *= base;
        }

        oflow_fix(&mut digits_vec);
    }

    digits_vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oflow_fix() {
        let mut digits_vec: Vec<u32> = vec![11, 12, 13];
        oflow_fix(&mut digits_vec);
        assert_eq!(digits_vec, vec![1, 3, 4, 1]);
    }

    #[test]
    fn test_oflow_factorial() {
        assert_eq!(oflow_factorial(10), 27);
        assert_eq!(oflow_factorial(100), 648);
    }

    #[test]
    fn test_oflow_pow() {
        assert_eq!(oflow_pow(2, 15), 26);
        assert_eq!(oflow_pow(2, 1000), 1366);
    }
}
