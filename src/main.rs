use rust_euler::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("\n27: {}", quadratic::quad_primes_count(1000, 1000));
}
