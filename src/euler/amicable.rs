use super::divisors::divs_get_prime;

pub fn am_sum(start: u64, limit: u64) -> u64 {
    let mut sum = 0;
    

    for a in start..limit {
        let b = divs_get_prime(a)
            .iter()
            .filter(|x| x != &&a)
            .sum();

        if a != b && b < limit {
            if a == divs_get_prime(b).iter().filter(|x| x != &&b).sum() {
                println!("{} {}", a, b);
                sum += a + b;
            }
        }
    }

    sum / 2
}
