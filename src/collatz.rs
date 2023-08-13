pub fn collatz(n: u64, count: usize, print: bool) -> (u64, usize) {
    if n == 1 {
        return (1, count);
    } else if n % 2 == 0 {
        return collatz(n / 2, count + 1, print);
    } else {
        return collatz((3 * n) + 1, count + 1, print);
    }
}

pub fn collatz_try(n: u64) -> u64 {
    let mut num_best = 0;
    let mut len_max = 0;

    let mut len_current;
    for i in (1..=n).rev() {
        len_current = collatz(i, 1, false).1;
        if len_current > len_max {
            len_max = len_current;
            num_best = i;
        }
    }
    println!("{} {}", num_best, len_max);

    num_best
}
