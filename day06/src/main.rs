use shared::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Todo {
    Toggle,
    TurnOn,
    TurnOff,
}

fn main() {
    let input: Vec<(Todo, Position, Position)> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let words = x.split(" ").flat_map(|x| x.split(",")).collect::<Vec<_>>();
            let todo: Todo = match words[0] {
                "toggle" => Todo::Toggle,
                "turn" => match words[1] {
                    "on" => Todo::TurnOn,
                    "off" => Todo::TurnOff,
                    _ => panic!("What"),
                },
                _ => panic!("What"),
            };

            let parts = words
                .iter()
                .map(|x| x.parse::<isize>())
                .flat_map(|x| x)
                .collect::<Vec<isize>>();

            (
                todo,
                Position {
                    x: parts[0],
                    y: parts[1],
                },
                Position {
                    x: parts[2],
                    y: parts[3],
                },
            )
        })
        .collect();

    {
        let mut lights = HashSet::new();
        for (todo, p1, p2) in input {
            for x in p1.x..=p2.x {
                for y in p1.y..=p2.y {
                    let c: (isize, isize) = (x, y);
                    match todo {
                        Todo::Toggle => {
                            if lights.contains(&c) {
                                lights.remove(&c);
                            } else {
                                lights.insert(c);
                            }
                        }
                        Todo::TurnOn => {
                            lights.insert(c);
                        }
                        Todo::TurnOff => {
                            lights.remove(&c);
                        }
                    }
                }
            }
        }
        println!("Part 1: {} lights", lights.len());
    }
}
