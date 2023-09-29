// util

pub fn grid_to_vec(grid: String) -> Vec<u64> {
    grid.split("")
        .filter_map(|x: &str| x.parse().ok())
        .collect()
}

pub fn grid_to_vec_fn(grid: String, delim: fn(char) -> bool) -> Vec<u64> {
    grid.split(delim)
        .filter_map(|x: &str| x.parse().ok())
        .collect()
}

pub fn grid_rows(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array: Vec<Vec<u64>> = vec![vec![0; n_2]; n_1];
    let mut i_1: usize;
    let mut i_2: usize;
    for i in 0..num_vec.len() {
        i_1 = i / n_2;
        i_2 = (i + n_2) % n_2;
        num_array[i_1][i_2] = num_vec[i];
    }

    num_array
}

pub fn grid_cols(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
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

pub fn grid_diags_neg(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
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

    num_array_lower.reverse();

    let mut num_array_full: Vec<Vec<u64>> = num_array_lower;
    num_array_full.append(&mut num_array_upper);

    num_array_full
}

pub fn grid_diags_pos(num_vec: &Vec<u64>, n_1: usize, n_2: usize) -> Vec<Vec<u64>> {
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_1];
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_2];

    for i in 0..n_1 {
        let mut current_diag: Vec<u64> = vec![];
        for j in (i..((i * n_1) + 1)).step_by(n_2 - 1) {
            current_diag.push(num_vec[j]);
        }

        num_array_upper[i] = current_diag;
    }

    for i in 0..n_2 {
        let mut current_diag: Vec<u64> = vec![];
        for j in ((n_2 * i + (n_2 - 1))..((num_vec.len() - n_2) + 1 + i)).step_by(n_2 - 1) {
            current_diag.push(num_vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    let mut num_array_full: Vec<Vec<u64>> = num_array_upper;
    num_array_full.append(&mut num_array_lower);

    num_array_full
}

// calc

pub fn grid_max_prod_win_vecs(num_array: Vec<Vec<u64>>, window_len: usize) -> u64 {
    let mut max_product: u64 = 0;
    let mut max_product_current: u64;
    for i in 0..num_array.len() {
        if num_array[i].len() >= window_len {
            max_product_current = num_array[i]
                .windows(window_len)
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

pub fn grid_cols_sums(grid: String) -> Vec<u64> {
    let num_array: Vec<Vec<u64>> = grid_cols(&grid_to_vec(grid), 100, 50);
    let mut sums: Vec<u64> = vec![0; num_array.len()];
    for (i, el) in num_array.iter().enumerate() {
        sums[i] = el.iter().sum();
    }

    sums
}

pub fn grid_sum_rows(grid: String) -> String {
    let mut sums_cols: Vec<u64> = grid_cols_sums(grid);
    let mut overflow: bool = true;
    sums_cols.reverse();

    let mut current: u64;
    while overflow == true {
        overflow = false;
        for i in 0..(sums_cols.len()) {
            current = sums_cols[i];
            if current >= 10 {
                if i == (sums_cols.len() - 1) {
                    sums_cols.push(0);
                }
                sums_cols[i] = current % 10;
                sums_cols[i + 1] += current / 10;

                overflow = true;
            }
        }
    }

    let mut digits: String = String::from("");

    for i in ((sums_cols.len() - 10)..(sums_cols.len())).rev() {
        digits.push_str(&sums_cols[i].to_string());
    }

    digits
}

//

pub fn grid_max_prod_win(grid: String, win: usize) -> u64 {
    grid_to_vec(grid)
        .windows(win)
        .map(|x: &[u64]| x.iter().product())
        .max()
        .unwrap()
}

pub fn grid_max_prod_win_all(
    grid: String,
    delim: fn(char) -> bool,
    n_1: usize,
    n_2: usize,
    window_len: usize,
) -> u64 {
    let num_vec: Vec<u64> = grid_to_vec_fn(grid, delim);

    let mut array: Vec<Vec<u64>> = vec![];
    array.append(&mut grid_rows(&num_vec, n_1, n_2));
    array.append(&mut grid_cols(&num_vec, n_1, n_2));
    array.append(&mut grid_diags_neg(&num_vec, n_1, n_2));
    array.append(&mut grid_diags_pos(&num_vec, n_1, n_2));

    grid_max_prod_win_vecs(array, window_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    // util

    #[test]
    fn test_grid_to_vec() {
        let grid = String::from(
            "
            01
            23
            ",
        );
        assert_eq!(grid_to_vec(grid), [0, 1, 2, 3])
    }

    #[test]
    fn test_grid_rows() {
        assert_eq!(grid_rows(&vec![0, 1, 2, 3], 2, 2), [[0, 1], [2, 3]]);
        assert_eq!(
            grid_rows(&vec![0, 1, 2, 3, 4, 5], 3, 2),
            [[0, 1], [2, 3], [4, 5]]
        );
        assert_eq!(
            grid_rows(&vec![0, 1, 2, 3, 4, 5], 2, 3),
            [[0, 1, 2], [3, 4, 5]]
        );
    }

    #[test]
    fn test_grid_cols() {
        assert_eq!(grid_cols(&vec![0, 1, 2, 3], 2, 2), [[0, 2], [1, 3]]);
        assert_eq!(
            grid_cols(&vec![0, 1, 2, 3, 4, 5], 3, 2),
            [[0, 2, 4], [1, 3, 5]]
        );
        assert_eq!(
            grid_cols(&vec![0, 1, 2, 3, 4, 5], 2, 3),
            [[0, 3], [1, 4], [2, 5]]
        );
    }

    #[test]
    fn test_grid_diags_neg() {
        assert_eq!(
            grid_diags_neg(&vec![0, 1, 2, 3], 2, 2),
            vec![vec![2], vec![0, 3], vec![0, 3], vec![1]]
        );
        assert_eq!(
            grid_diags_neg(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3),
            vec![
                vec![6],
                vec![3, 7],
                vec![0, 4, 8],
                vec![0, 4, 8],
                vec![1, 5],
                vec![2]
            ]
        );
    }

    #[test]
    fn test_grid_diags_pos() {
        assert_eq!(
            grid_diags_pos(&vec![0, 1, 2, 3], 2, 2),
            vec![vec![0], vec![1, 2], vec![1, 2], vec![3]]
        );
        assert_eq!(
            grid_diags_pos(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3),
            vec![
                vec![0],
                vec![1, 3],
                vec![2, 4, 6],
                vec![2, 4, 6],
                vec![5, 7],
                vec![8]
            ]
        );
    }

    // calc

    #[test]
    fn test_grid_max_prod_win() {
        assert_eq!(grid_max_prod_win(data::data_08(), 4), 5832);
        assert_eq!(grid_max_prod_win(data::data_08(), 13), 23514624000);
    }

    #[test]
    fn test_grid_max_prod_win_all() {
        assert_eq!(
            grid_max_prod_win_all(data::data_11(), char::is_whitespace, 20, 20, 4),
            70600674
        );
    }
}
