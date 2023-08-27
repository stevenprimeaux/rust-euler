use std::collections::HashMap;

pub fn collatz_try(n: u64) -> u64 {
    let mut best: u64 = 0;
    let mut len_max: usize = 0;
    let mut counts: HashMap<u64, usize> = HashMap::from([(1, 1)]);

    let mut len_current: usize;
    for i in (n / 2)..=n {
        len_current = collatz_map(i, &mut counts);
        if len_current > len_max {
            len_max = len_current;
            best = i;
        }
    }

    best
}

#[allow(dead_code)]
pub fn collatz_brute(n: u64, count: usize) -> (u64, usize) {
    if n == 1 {
        return (1, count);
    } else if n % 2 == 0 {
        return collatz_brute(n / 2, count + 1);
    } else {
        return collatz_brute(3 * n + 1, count + 1);
    }
}

pub fn collatz_map(n: u64, counts: &mut HashMap<u64, usize>) -> usize {
    if counts.get(&n).is_some() {
        return counts[&n];
    }

    let count: usize;
    if n % 2 == 0 {
        count = 1 + collatz_map(n / 2, counts);
    } else {
        count = 2 + collatz_map((3 * n + 1) / 2, counts);
    }
    counts.insert(n, count);

    counts[&n]
}
