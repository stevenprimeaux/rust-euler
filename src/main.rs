mod data;
mod fibonacci;
mod grid;
mod multiples;
mod palindrome;
mod primes;
mod pythagorean;
mod squares;
mod triangular;
mod util;

use crate::data::*;

fn main() {
    println!("01: {}", multiples::mult_sum_2(3, 5, 1000));
    println!("02: {}", fibonacci::fib_sum_even(4000000));
    println!("03: {}", primes::prime_fact_max(600851475143));
    println!("04: {}", palindrome::pal_product_3());
    println!("05: {}", multiples::mult_smallest(20));
    println!("06: {}", squares::sq_diff(100));
    println!("07: {}", primes::prime_nth(10001));
    println!("08: {}", grid::grid_prod_max_rowwise(data_08(), 13));
    println!("09: {}", pythagorean::pythag_triple(1000));
    println!("10: {}", primes::prime_sum(2000000));
    println!("11: {}", grid::grid_prod_max_alldir(data_11(), 20, 20, 4));
    println!("12: {}", triangular::tri_sum(500));
    println!("13: {}", grid::grid_sum_rows(data_13()));
}
