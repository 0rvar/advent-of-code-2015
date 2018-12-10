use shared::*;

fn main() {
    // let line_regex = Regex::new(
    //     r"\[1518-(\d+)-(\d+) (\d+):(\d+)\] (Guard #\d+ begins shift|falls asleep|wakes up)",
    // )
    // .unwrap();
    // let guard_regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    // for line in lines {
    //     let captures = line_regex.captures(line).unwrap();
    //     let month = get_number_match(&captures, 1);
    let input: Vec<(usize, usize, usize)> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let parts = x
                .split("x")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();

            (parts[0], parts[1], parts[2])
        })
        .collect();

    let wrapping_paper_needed: usize = input
        .iter()
        .map(|(l, w, h)| {
            let sides = [(l * w), (w * h), (h * l)];
            let paper: usize = 2 * sides.iter().sum::<usize>();
            let slack: usize = *sides.iter().min().unwrap();
            paper + slack
        })
        .sum::<usize>();
    println!("Part 1: {}", wrapping_paper_needed);
}
