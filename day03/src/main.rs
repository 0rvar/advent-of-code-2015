use shared::*;
use std::collections::HashMap;

fn main() {
    let input: Vec<Direction> = include_str!("input.txt")
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Direction::West,
            '>' => Direction::East,
            '^' => Direction::North,
            'v' => Direction::South,
            x => panic!(format!("Unknown direction {}", x)),
        })
        .collect();

    let mut houses: HashMap<Position, usize> = HashMap::new();
    let mut pos = Position { x: 0, y: 0 };

    for dir in input {
        let next_pos = move_in_direction(&pos, &dir);
        *houses.entry(pos).or_insert(0) += 1;
        pos = next_pos;
    }
    println!("Part 1: {}", houses.len());
}
