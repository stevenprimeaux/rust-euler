use std::collections::HashMap;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Word<'a> {
    word: &'a str,
    letters: Vec<&'a str>,
    letters_vals: Vec<u64>,
    position: u64,
    total: u64,
}

impl Word<'_> {
    fn new<'a>(word: &str, position: u64) -> Word {
        let mut letters: Vec<&str> = vec![];
        // println!("{}", word.chars().count());
        let mut letters_vals: Vec<u64> = vec![1; word.chars().count()];
        for (i, c) in word.chars().map(|x| x.to_string()).enumerate() {
            // println!("{}", &c);
            // letters.push(&c);

            letters_vals[i] = lu_letters_vals(&c);
        }

        let letters_sum: u64 = letters_vals.iter().sum();
        let total: u64 = letters_sum * position;

        Word {
            word,
            letters,
            letters_vals,
            position,
            total,
        }
    }
}

pub fn txt_names_sum(p: &Path) -> u64 {
    let f = fs::read_to_string(p).unwrap();
    let mut vec: Vec<String> = f
        .split(",")
        .map(|x| x.trim_matches('"'))
        .map(|x| x.to_lowercase())
        .collect();

    vec.sort();

    let mut words: Vec<Word> = vec![Word::new(&"a", 0); vec.len()];

    for (i, v) in vec.iter().enumerate() {
        words[i] = Word::new(v, (i + 1) as u64);
    }

    // for w in &words {
    //     println!("{} {}", w.position, w.word);
    //     for l in &w.letters {
    //         print!("{} ", l);
    //     }
    //     for v in &w.letters_vals {
    //         print!("{} ", v);
    //     }
    //     println!("{}", w.total);
    // }

    let mut sum = 0;

    for w in words {
        sum += w.total;
    }

    sum
}

pub fn lu_letters_vals(k: &str) -> u64 {
    let letters_vals_map: HashMap<&str, u64> = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
    ]);

    letters_vals_map[&k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_names_sum() {
        assert_eq!(txt_names_sum("data/names.txt".as_ref()), 871198282);
    }
}
