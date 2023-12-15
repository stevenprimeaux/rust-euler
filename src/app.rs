use crate::*;

#[cfg(not(tarpaulin_include))]
pub fn run() {
    // println!("15: {}", paths::path_count(20));
    // println!("18: {}", paths::path_sum_tri(data_18()));
    // println!(
    //     "22: {}",
    //     text::txt_names_sum("https://projecteuler.net/resources/documents/0022_names.txt")
    // );
    // println!("\n25: {}", rust_euler::fibonacci::fib_oflow(1000));

    println!("\n26: {}", decimal::dec_period_max(1000));
}
