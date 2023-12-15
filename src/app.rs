use crate::decimal;

#[cfg(not(tarpaulin_include))]
pub fn run() {
    // println!("15: {}", paths::path_count(20));
    // println!("18: {}", paths::path_sum_tri(data_18()));
    // println!("19: {}", calendar::cal_count_sundays(1901, 2000, 2));
    // println!("21: {}", amicable::ami_sum(10000));
    // println!(
    //     "22: {}",
    //     text::txt_names_sum("https://projecteuler.net/resources/documents/0022_names.txt")
    // );
    // println!("23: {}", abundant::abund_sum_addends_notabundant(28123))

    // println!("\n25: {}", rust_euler::fibonacci::fib_oflow(1000));

    println!("\n26: {}", decimal::dec_period_max(1000));
}
