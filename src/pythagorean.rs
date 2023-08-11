use crate::util;

pub fn pythag_triple(s: u64) -> u64 {
    let s_2: u64 = s / 2;
    let mut s_m: u64;

    let mut k: u64;
    let m_limit: u64 = ((s as f64).sqrt().ceil() - 1.0) as u64;

    let d: u64;
    let n: u64;

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;

    'outer: for m in 2..m_limit {
        if s_2 % m == 0 {
            s_m = s_2 / m;
            while s_m % 2 == 0 {
                s_m /= 2;
            }

            if m % 2 == 1 {
                k = m + 2;
            } else {
                k = m + 1;
            }

            while (k < 2 * m) & (k <= s_m) {
                if (s_m % k == 0) & (util::gcd(k, m) == 1) {
                    d = s_2 / (k * m);
                    n = k - m;

                    a = d * (m * m - n * n);
                    b = d * (2 * m * n);
                    c = d * (m * m + n * n);

                    break 'outer;
                }
                k += 2;
            }
        }
    }
    a * b * c
}
