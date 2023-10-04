use rust_euler::euler::*;
// use rust_euler::data::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    // println!("15: {}", paths::path_count(20));
    // println!("18: {}", paths::path_sum_tri(data_18()));
    // println!("19: {}", calendar::cal_count_sundays(1901, 2000, 2));
    // println!("21: {}", amicable::ami_sum(10000));

    println!(
        "22: {}",
        text::txt_names_sum("https://projecteuler.net/resources/documents/0022_names.txt")
    );
}
