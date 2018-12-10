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

    {
        let mut houses: HashMap<Position, usize> = HashMap::new();
        let mut pos = Position { x: 0, y: 0 };
        houses.insert(pos.clone(), 1);

        for dir in &input {
            *houses.entry(pos.clone()).or_insert(0) += 1;
            pos = move_in_direction(&pos, &dir);
        }
        println!("Part 1: {}", houses.len());
    }

    {
        let mut houses: HashMap<Position, usize> = HashMap::new();
        let mut pos1 = Position { x: 0, y: 0 };
        let mut pos2 = pos1.clone();
        houses.insert(pos1.clone(), 2);

        for (i, dir) in input.iter().enumerate() {
            if i % 2 == 0 {
                *houses.entry(pos1.clone()).or_insert(0) += 1;
                pos1 = move_in_direction(&pos1, &dir);
            } else {
                *houses.entry(pos2.clone()).or_insert(0) += 1;
                pos2 = move_in_direction(&pos2, &dir);
            }
        }
        println!("Part 2: {}", houses.len());
    }
}
