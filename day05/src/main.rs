use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("input.txt").trim().split("\n").collect();

    println!(
        "Part 1: {}",
        input.iter().filter(|&x| nice_string_1(x)).count()
    );

    println!(
        "Part 2: {}",
        input
            .iter()
            .filter(|&x| nice_string_2(x.as_bytes()))
            .count()
    );
}

fn nice_string_2(s: &[u8]) -> bool {
    let mut pairs = HashMap::new();
    let mut has_duplicate_pair = false;
    let mut has_repeating = false;
    for i in 0..(s.len() - 1) {
        let next_3: Vec<u8> = s[i..].iter().take(3).map(|x| *x).collect::<Vec<_>>();
        if next_3.len() == 3 && next_3[0] == next_3[2] {
            has_repeating = true;
        }
        let pair_id = next_3
            .iter()
            .take(2)
            .map(|x| *x as char)
            .collect::<String>();
        let pair_entry = pairs.entry(pair_id).or_insert(i);
        if *pair_entry + 1 < i {
            has_duplicate_pair = true;
        }
    }

    has_duplicate_pair && has_repeating
}

#[test]
fn test_nice_string_2() {
    assert_eq!(nice_string_2("xxyxx".as_bytes()), true);
    assert_eq!(nice_string_2("qjhvhtzxzqqjkmpb".as_bytes()), true);
    assert_eq!(nice_string_2("uurcxstgmygtbstg".as_bytes()), false);
    assert_eq!(nice_string_2("ieodomkazucvgmuy".as_bytes()), false);
}

const VOWELS: &str = "aeiou";
const NAIGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];
fn nice_string_1(s: &str) -> bool {
    let mut has_double = false;
    let mut vowel_count = 0;
    for (c, next) in s.chars().zip(format!("{}0", s).chars().skip(1)) {
        let combined: &str = &format!("{}{}", c, next);
        if NAIGHTY_STRINGS.contains(&combined) {
            return false;
        }
        if c == next {
            has_double = true;
        }
        if VOWELS.contains(c) {
            vowel_count += 1;
        }
    }
    has_double && vowel_count >= 3
}

#[test]
fn test_nice_string_1() {
    assert_eq!(nice_string_1("ugknbfddgicrmopn"), true);
    assert_eq!(nice_string_1("aaa"), true);
    assert_eq!(nice_string_1("jchzalrnumimnmhp"), false);
    assert_eq!(nice_string_1("haegwjzuvuyypxyu"), false);
    assert_eq!(nice_string_1("dvszwmarrgswjxmb"), false);
}
