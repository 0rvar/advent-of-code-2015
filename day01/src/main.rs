fn main() {
    let input = include_str!("input.txt").trim();

    let mut floor = 0isize;
    let mut first_basement = None;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            x => panic!(format!("Unknown char {}", x)),
        }
        if floor == -1 && first_basement == None {
            first_basement = Some(i + 1);
        }
    }
    println!("Part 1: {}", floor);
    println!("Part 2: {}", first_basement.unwrap());
}
