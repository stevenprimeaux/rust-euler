use std::collections::HashMap;

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

pub fn path_sum_tri(tri: String) -> u64 {
    let rows = tri.trim().split("\n");
    let mut array: Vec<Vec<u64>> = vec![];
    let mut array_spaced: Vec<Vec<u64>> = vec![vec![0; 29]; 15];

    let mut row;
    for r in rows {
        row = r
            .trim()
            .split_whitespace()
            .filter_map(|s: &str| s.parse().ok())
            .collect::<Vec<u64>>();

        array.push(row);
    }

    for i in 0..=14 {
        let index_start = 14 - i;
        let index_end = index_start + (2 * i) + 1;
        let indices = (index_start..index_end).step_by(2);
        for (j, v) in indices.enumerate() {
            array_spaced[i][v] = array[i][j];
        }
    }

    let mut nodes: HashMap<Node, Vec<u64>> = HashMap::from([(Node { row: 0, col: 0 }, vec![0])]);
    let current_row: usize = 14;
    for (i, element) in array_spaced[current_row].iter().enumerate() {
        if *element != 0 {
            let current_node = Node {
                row: current_row,
                col: i,
            };
            nodes.insert(current_node, vec![*element]);
        }
    }

    for current_row in (0..14).rev() {
        for (i, element) in array_spaced[current_row].iter().enumerate() {
            if *element != 0 {
                let left_key = Node {
                    row: current_row + 1,
                    col: i - 1,
                };
                let left_path = nodes.get(&left_key).unwrap();
                let left_sum: u64 = left_path.iter().sum();
                let right_key = Node {
                    row: current_row + 1,
                    col: i + 1,
                };
                let right_path = nodes.get(&right_key).unwrap();
                let right_sum: u64 = right_path.iter().sum();

                let mut current_path: Vec<u64> = vec![0; left_path.len()];
                if right_sum > left_sum {
                    current_path.copy_from_slice(&right_path);
                } else {
                    current_path.copy_from_slice(&left_path);
                }
                current_path.push(*element);

                let current_node = Node {
                    row: current_row,
                    col: i,
                };
                nodes.insert(current_node, current_path);
            }
        }
    }

    let mut best_sum: u64 = 0;
    for node in nodes {
        if node.0.row == 0 {
            for e in node.1 {
                best_sum += e;
            }
        }
    }

    best_sum
}

#[derive(Hash, Eq, PartialEq)]
struct Node {
    row: usize,
    col: usize,
}
