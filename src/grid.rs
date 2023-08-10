pub fn make_2d_row(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array: Vec<Vec<u64>> = vec![vec![0; n_2]; n_1];
    let mut i_1: usize;
    let mut i_2: usize;
    for i in 0..num_vec.len() {
        i_1 = i / n_1;
        i_2 = i - (i_1 * n_1);
        num_array[i_1][i_2] = num_vec[i];
    }

    num_array
}

pub fn make_2d_col(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array: Vec<Vec<u64>> = vec![vec![0; n_1]; n_2];
    let mut i_1: usize;
    let mut i_2: usize;
    for i in 0..num_vec.len() {
        i_1 = i / n_2;
        i_2 = i - (i_1 * n_2);
        num_array[i_2][i_1] = num_vec[i];
    }

    num_array
}

pub fn make_2d_diag_neg(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_1];
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_2];

    for i in 0..n_1 {
        let mut current_diag: Vec<u64> = vec![];
        for j in ((i * n_1)..(num_vec.len())).step_by(n_2 + 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    for i in 0..n_2 {
        let mut current_diag: Vec<u64> = vec![];
        for j in (i..(num_vec.len() - (i * n_2))).step_by(n_2 + 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_upper[i] = current_diag;
    }

    let mut num_array_full: Vec<Vec<u64>> = num_array_lower;
    num_array_full.append(&mut num_array_upper);

    num_array_full
}

pub fn make_2d_diag_pos(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_1];
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_2];

    for i in 0..n_1 {
        let mut current_diag: Vec<u64> = vec![];
        for j in (i..(((i + 1) * n_1) - 1)).step_by(n_2 - 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_upper[i] = current_diag;
    }

    for i in 0..n_2 {
        let mut current_diag: Vec<u64> = vec![];
        for j in ((n_2 * i + (n_2 - 1))..(num_vec.len())).step_by(n_2 - 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    let mut num_array_full: Vec<Vec<u64>> = num_array_upper;
    num_array_full.append(&mut num_array_lower);

    num_array_full
}

pub fn largest_product_array(num_array: Vec<Vec<u64>>, len_window: usize) -> u64 {
    let mut max_product: u64 = 0;
    let mut max_product_current: u64;
    for i in 0..num_array.len() {
        if num_array[i].len() >= len_window {
            max_product_current = num_array[i]
                .windows(len_window)
                .map(|x: &[u64]| x.iter().product())
                .max()
                .unwrap();

            if max_product_current > max_product {
                max_product = max_product_current;
            }
        }
    }

    max_product
}

pub fn get_max_row(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_row(&num_vec, n_1, n_2), 4)
}

pub fn get_max_col(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_col(&num_vec, n_1, n_2), 4)
}

pub fn get_max_diag_neg(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_diag_neg(&num_vec, n_1, n_2), 4)
}

pub fn get_max_diag_pos(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> u64 {
    largest_product_array(make_2d_diag_pos(&num_vec, n_1, n_2), 4)
}

pub fn largest_product_grid(grid: String, n_1: usize, n_2: usize) -> u64 {
    let num_vec: Vec<u64> = grid
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let maxes: [u64; 4] = [
        get_max_row(&num_vec, n_1, n_2),
        get_max_col(&num_vec, n_1, n_2),
        get_max_diag_neg(&num_vec, n_1, n_2),
        get_max_diag_pos(&num_vec, n_1, n_2),
    ];

    *maxes.iter().max().unwrap()
}

pub fn grid_to_vec(grid: String) -> Vec<u64> {
    grid.split("")
        .filter_map(|s: &str| s.parse().ok())
        .collect::<Vec<u64>>()
}

pub fn grid_sum_rows(grid: String) -> u64 {
    let mut num_array: Vec<Vec<u64>> = make_2d_col(&grid_to_vec(grid), 100, 50);
    num_array.reverse();

    let mut col_sums: Vec<u64> = vec![0; num_array.len()];
    for (i, el) in num_array.iter().enumerate() {
        col_sums[i] = el.iter().sum();
        // println!("{}", col_sums[i]);
    }

    /*

    */

    num_array.len() as u64
}
