/// Returns nth permutation (in lexicographic order) of a given vector.
///
/// # Examples
///
/// ```
/// use rust_euler::euler::permutation;
///
/// assert_eq!(permutation::perm_lex(vec![0, 1, 2], 1), "012");
/// assert_eq!(permutation::perm_lex(vec![0, 1, 2], 2), "021");
/// assert_eq!(permutation::perm_lex(vec![0, 1, 2], 3), "102");
/// assert_eq!(permutation::perm_lex(vec![0, 1, 2], 4), "120");
/// ```
pub fn perm_lex(mut vec: Vec<u64>, n: u64) -> String {
    let mut k: Option<usize>;
    let mut l: usize;
    let mut a_k: u64;

    let mut count: u64 = 0;
    let mut perm_n: String = String::from("");

    loop {
        count += 1;

        if count == n {
            for e in &vec {
                perm_n.push_str(&e.to_string());
            }
            return perm_n;
        }

        k = vec
            .windows(2)
            .map(|x: &[u64]| x[0] < x[1])
            .rposition(|x: bool| x);

        match k {
            None => return String::from(""),
            Some(k) => {
                a_k = vec[k];

                l = vec.iter().map(|x| x > &a_k).rposition(|x| x).unwrap();

                vec.swap(k, l);

                vec[(k + 1)..].reverse();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm_lex() {
        assert_eq!(
            perm_lex(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1000000),
            "2783915460"
        );
    }
}
