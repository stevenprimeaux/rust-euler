use rust_euler::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("\n28: {}", spiral::spiral_diag_sum(1001));
}
