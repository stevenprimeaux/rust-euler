// grid_util

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

pub fn grid_rows(vec: &Vec<u64>, n_rows: usize, n_cols: usize) -> Vec<Vec<u64>> {
    let mut array: Vec<Vec<u64>> = vec![vec![0; n_cols]; n_rows];
    let mut i_row: usize;
    let mut i_col: usize;
    for n in 0..vec.len() {
        i_row = n / n_cols;
        i_col = n - (i_row * n_cols);
        array[i_row][i_col] = vec[n];
    }

    array
}

pub fn grid_cols(vec: &Vec<u64>, n_rows: usize, n_cols: usize) -> Vec<Vec<u64>> {
    let mut array: Vec<Vec<u64>> = vec![vec![0; n_rows]; n_cols];
    let mut i_row: usize;
    let mut i_col: usize;
    for n in 0..vec.len() {
        i_row = n / n_cols;
        i_col = n - (i_row * n_cols);
        array[i_col][i_row] = vec[n];
    }

    array
}

pub fn grid_diags_neg(vec: &Vec<u64>, n_rows: usize, n_cols: usize) -> Vec<Vec<u64>> {
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_rows];
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_cols];
    let mut current_diag: Vec<u64>;

    for i in 0..n_rows {
        current_diag = vec![];
        for j in ((i * n_rows)..(vec.len())).step_by(n_cols + 1) {
            current_diag.push(vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    for i in 0..n_cols {
        current_diag = vec![];
        for j in (i..(vec.len() - (i * n_cols))).step_by(n_cols + 1) {
            current_diag.push(vec[j]);
        }
        num_array_upper[i] = current_diag;
    }

    num_array_lower.reverse();

    let mut num_array_full: Vec<Vec<u64>> = num_array_lower;
    num_array_full.append(&mut num_array_upper);

    num_array_full.dedup();

    num_array_full
}

pub fn grid_diags_pos(vec: &Vec<u64>, n_rows: usize, n_cols: usize) -> Vec<Vec<u64>> {
    let mut num_array_upper: Vec<Vec<u64>> = vec![vec![]; n_rows];
    let mut num_array_lower: Vec<Vec<u64>> = vec![vec![]; n_cols];

    for i in 0..n_rows {
        let mut current_diag: Vec<u64> = vec![];
        for j in (i..((i * n_rows) + 1)).step_by(n_cols - 1) {
            current_diag.push(vec[j]);
        }

        num_array_upper[i] = current_diag;
    }

    for i in 0..n_cols {
        let mut current_diag: Vec<u64> = vec![];
        for j in ((n_cols * i + (n_cols - 1))..((vec.len() - n_cols) + 1 + i)).step_by(n_cols - 1) {
            current_diag.push(vec[j]);
        }
        num_array_lower[i] = current_diag;
    }

    let mut num_array_full: Vec<Vec<u64>> = num_array_upper;
    num_array_full.append(&mut num_array_lower);

    num_array_full.dedup();

    num_array_full
}

//

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

// grid_calc

pub fn grid_max_prod_win(grid: String, win: usize) -> u64 {
    grid_to_vec(grid)
        .windows(win)
        .map(|x: &[u64]| x.iter().product())
        .max()
        .unwrap()
}

pub fn grid_max_prod_win_vecs(array: Vec<Vec<u64>>, win: usize) -> u64 {
    let mut max_prod: u64 = 0;
    let mut max_prod_current: u64;
    for v in array {
        if v.len() >= win {
            max_prod_current = v
                .windows(win)
                .map(|x: &[u64]| x.iter().product())
                .max()
                .unwrap();

            if max_prod_current > max_prod {
                max_prod = max_prod_current;
            }
        }
    }

    max_prod
}

pub fn grid_max_prod_win_all(
    grid: String,
    delim: fn(char) -> bool,
    n_rows: usize,
    n_cols: usize,
    win: usize,
) -> u64 {
    let vec: Vec<u64> = grid_to_vec_fn(grid, delim);

    let mut array: Vec<Vec<u64>> = vec![];
    array.append(&mut grid_rows(&vec, n_rows, n_cols));
    array.append(&mut grid_cols(&vec, n_rows, n_cols));
    array.append(&mut grid_diags_neg(&vec, n_rows, n_cols));
    array.append(&mut grid_diags_pos(&vec, n_rows, n_cols));

    grid_max_prod_win_vecs(array, win)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    // grid_util

    #[test]
    fn test_grid_to_vec() {
        let grid = String::from(
            "
            01
            23
            ",
        );
        assert_eq!(grid_to_vec(grid), [0, 1, 2, 3]);
    }

    #[test]
    fn test_grid_to_vec_fn() {
        let grid = String::from(
            "
            00 01 02 03
            04 05 06 07
            08 09 10 11
            ",
        );
        assert_eq!(
            grid_to_vec_fn(grid, char::is_whitespace),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
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
            vec![vec![2], vec![0, 3], vec![1]]
        );
        assert_eq!(
            grid_diags_neg(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3),
            vec![vec![6], vec![3, 7], vec![0, 4, 8], vec![1, 5], vec![2]]
        );
    }

    #[test]
    fn test_grid_diags_pos() {
        assert_eq!(
            grid_diags_pos(&vec![0, 1, 2, 3], 2, 2),
            vec![vec![0], vec![1, 2], vec![3]]
        );
        assert_eq!(
            grid_diags_pos(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3, 3),
            vec![vec![0], vec![1, 3], vec![2, 4, 6], vec![5, 7], vec![8]]
        );
    }

    // grid_calc

    #[test]
    fn test_grid_max_prod_win() {
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
