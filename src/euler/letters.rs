use crate::util;
use std::collections::HashMap;

pub fn letters_build_phrase(n: u32) -> String {
    let counts: HashMap<u32, usize> = util::counts_10(n);
    let mut phrase: String = String::from("");

    if counts[&1000] >= 1 {
        phrase.push_str(&format!("{} thousand, ", counts[&1000]));
    }
    if counts[&100] >= 1 {
        phrase.push_str(&format!("{} hundred", counts[&100]));
        if counts[&10] >= 1 || counts[&1] >= 1 {
            phrase.push_str(" and, ");
        } else {
            phrase.push_str(", ");
        }
    }
    if counts[&10] >= 1 {
        phrase.push_str(&format!("{} ten, ", counts[&10]));
    }
    if counts[&1] >= 1 {
        phrase.push_str(&format!("{}", counts[&1]));
    }

    for sub in letters_sub_teens() {
        phrase = phrase.replace(sub.0, sub.1);
    }
    for sub in letters_sub_tens() {
        phrase = phrase.replace(sub.0, sub.1);
    }
    for sub in letters_sub_ones() {
        phrase = phrase.replace(sub.0, sub.1);
    }

    phrase = phrase.replace("one ten", "ten");
    phrase = phrase.replace(',', "");

    phrase
}

pub fn letters_count_chars(phrase: &str) -> usize {
    phrase.replace(['-', ' '], "").chars().count()
}

pub fn letters_count_chars_cum(limit: u32) -> usize {
    let mut sum: usize = 0;
    let mut count: usize;
    let mut phrase: String;
    for i in 1..=limit {
        phrase = letters_build_phrase(i);
        count = letters_count_chars(&phrase);
        sum += count;
    }

    sum
}

#[cfg(not(tarpaulin_include))]
pub fn letters_sub_teens() -> Vec<(&'static str, &'static str)> {
    vec![
        ("1 ten, 1", "eleven"),
        ("1 ten, 2", "twelve"),
        ("1 ten, 3", "thirteen"),
        ("1 ten, 4", "fourteen"),
        ("1 ten, 5", "fifteen"),
        ("1 ten, 6", "sixteen"),
        ("1 ten, 7", "seventeen"),
        ("1 ten, 8", "eighteen"),
        ("1 ten, 9", "nineteen"),
    ]
}

#[cfg(not(tarpaulin_include))]
pub fn letters_sub_tens() -> Vec<(&'static str, &'static str)> {
    vec![
        ("2 ten", "twenty"),
        ("3 ten", "thirty"),
        ("4 ten", "forty"),
        ("5 ten", "fifty"),
        ("6 ten", "sixty"),
        ("7 ten", "seventy"),
        ("8 ten", "eighty"),
        ("9 ten", "ninety"),
    ]
}

#[cfg(not(tarpaulin_include))]
pub fn letters_sub_ones() -> Vec<(&'static str, &'static str)> {
    vec![
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letters_build_phrase() {
        assert_eq!(letters_build_phrase(1), "one");
        assert_eq!(letters_build_phrase(12), "twelve");
        assert_eq!(letters_build_phrase(123), "one hundred and twenty three");
        assert_eq!(
            letters_build_phrase(1234),
            "one thousand two hundred and thirty four"
        );
    }

    #[test]
    fn test_letters_count_chars() {
        assert_eq!(letters_count_chars("one hundred and fifteen"), 20);
        assert_eq!(letters_count_chars("three hundred and forty-two"), 23);
    }

    #[test]
    fn test_letters_count_chars_cum() {
        assert_eq!(letters_count_chars_cum(5), 19);
        assert_eq!(letters_count_chars_cum(1000), 21124);
    }
}
