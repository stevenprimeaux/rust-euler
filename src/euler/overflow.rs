pub fn oflow_pow(base: u32, power: u32) -> u32 {
    let mut digits_vec: Vec<u32> = vec![base];
    let mut digits_sum: u32 = 0;

    let mut overflow: bool;
    let mut current: u32;

    for _ in 2..=power {
        for n in 0..digits_vec.len() {
            digits_vec[n] *= base;
        }

        overflow = true;

        while overflow == true {
            overflow = false;
            for i in 0..(digits_vec.len()) {
                current = digits_vec[i];
                if current >= 10 {
                    if i == (digits_vec.len() - 1) {
                        digits_vec.push(0);
                    }
                    digits_vec[i] = current % 10;
                    digits_vec[i + 1] += current / 10;

                    overflow = true;
                }
            }
        }
    }

    for d in digits_vec {
        digits_sum += d;
    }

    digits_sum
}

pub fn oflow_factorial(n: u32) -> u32 {
    let mut digits_vec: Vec<u32> = vec![1];
    let mut digits_sum: u32 = 0;

    let mut overflow: bool;
    let mut current: u32;

    for n_current in 1..=n {
        for i in 0..digits_vec.len() {
            digits_vec[i] *= n_current;
        }

        overflow = true;

        while overflow == true {
            overflow = false;
            for i in 0..(digits_vec.len()) {
                current = digits_vec[i];
                if current >= 10 {
                    if i == (digits_vec.len() - 1) {
                        digits_vec.push(0);
                    }
                    digits_vec[i] = current % 10;
                    digits_vec[i + 1] += current / 10;

                    overflow = true;
                }
            }
        }
    }

    for d in digits_vec {
        digits_sum += d;
    }

    digits_sum
}
