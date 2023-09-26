use rust_euler::euler::*;
// use rust_euler::data::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    // println!("08: {}", grid::grid_prod_max_rowwise(data_08(), 13));
    // println!("11: {}", grid::grid_prod_max_alldir(data_11(), 20, 20, 4));
    // println!("13: {}", grid::grid_sum_rows(data_13()));
    // println!("15: {}", paths::path_count(20));
    // println!("18: {}", paths::path_sum_tri(data_18()));
    // println!("19: {}", calendar::cal_count_sundays(1901, 2000, 2));
    // println!("20: {}", overflow::oflow_factorial(100));

    println!("21: {}", amicable::ami_sum(10000));
}
