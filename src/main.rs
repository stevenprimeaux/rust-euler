use rust_euler::data::*;
use rust_euler::euler::*;

fn main() {
    println!("08: {}", grid::grid_prod_max_rowwise(data_08(), 13));
    println!("11: {}", grid::grid_prod_max_alldir(data_11(), 20, 20, 4));
    println!("13: {}", grid::grid_sum_rows(data_13()));
    println!("15: {}", paths::path_count(20));
    println!("16: {}", overflow::oflow_pow(2, 1000));
    println!("17: {}", letters::letters_count_chars_cum(1000));
    println!("18: {}", paths::path_sum_tri(data_18()));
}
