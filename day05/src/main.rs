fn main() {
    let input: Vec<&str> = include_str!("input.txt").trim().split("\n").collect();

    println!(
        "Part 1: {}",
        input.iter().filter(|&x| nice_string(x)).count()
    );
}

const VOWELS: &str = "aeiou";
const NAIGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];
fn nice_string(s: &str) -> bool {
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
fn test_nice_string() {
    assert_eq!(nice_string("ugknbfddgicrmopn"), true);
    assert_eq!(nice_string("aaa"), true);
    assert_eq!(nice_string("jchzalrnumimnmhp"), false);
    assert_eq!(nice_string("haegwjzuvuyypxyu"), false);
    assert_eq!(nice_string("dvszwmarrgswjxmb"), false);
}
