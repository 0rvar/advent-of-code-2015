fn main() {
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

    let ribbon_needed: usize = input.iter().map(ribbon).sum::<usize>();
    println!("Part 2: {}", ribbon_needed);
}

fn ribbon(&(l, w, h): &(usize, usize, usize)) -> usize {
    let mut sides: Vec<usize> = vec![l, w, h];
    sides.sort();
    let wrap = (sides[0] + sides[1]) * 2;
    let bow = sides.iter().product::<usize>();
    wrap + bow
}

#[test]
fn test_ribbon() {
    assert_eq!(ribbon(&(2, 3, 4)), 34);
}
