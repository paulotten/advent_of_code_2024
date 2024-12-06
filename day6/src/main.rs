mod input;

use input::*;
use std::collections::HashSet;

fn main() {
    part1();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let MapInfo {
        max,
        mut guard_pos,
        mut guard_dir,
        obstructions,
    } = get_map_info(input);

    let mut walked: HashSet<Position> = HashSet::new();

    while guard_pos.x > 0 && guard_pos.y > 0 && guard_pos.x <= max.x && guard_pos.y <= max.y {
        walked.insert(Position {
            x: guard_pos.x,
            y: guard_pos.y,
        });

        match guard_dir {
            Direction::Up => {
                if obstructions.contains(&Position {
                    x: guard_pos.x,
                    y: guard_pos.y - 1,
                }) {
                    guard_dir.turn();
                } else {
                    guard_pos.y -= 1;
                }
            }
            Direction::Right => {
                if obstructions.contains(&Position {
                    x: guard_pos.x + 1,
                    y: guard_pos.y,
                }) {
                    guard_dir.turn();
                } else {
                    guard_pos.x += 1;
                }
            }
            Direction::Down => {
                if obstructions.contains(&Position {
                    x: guard_pos.x,
                    y: guard_pos.y + 1,
                }) {
                    guard_dir.turn();
                } else {
                    guard_pos.y += 1;
                }
            }
            Direction::Left => {
                if obstructions.contains(&Position {
                    x: guard_pos.x - 1,
                    y: guard_pos.y,
                }) {
                    guard_dir.turn();
                } else {
                    guard_pos.x -= 1;
                }
            }
        }
    }

    println!("{}", walked.len());
}

#[derive(Debug)]
struct MapInfo {
    max: Position,
    guard_pos: Position,
    guard_dir: Direction,
    obstructions: HashSet<Position>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_map_info(input: &str) -> MapInfo {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_dir = Direction::Up;
    let mut obstructions = HashSet::new();

    let mut y = 0;
    for line in input.lines() {
        max_y = y;

        if y == 0 {
            max_x = line.len() as i32;
        }

        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => {}
                '#' => {
                    obstructions.insert(Position { x, y });
                }
                '^' => {
                    guard_dir = Direction::Up;
                    guard_x = x;
                    guard_y = y;
                }
                '>' => {
                    guard_dir = Direction::Right;
                    guard_x = x;
                    guard_y = y;
                }
                'v' => {
                    guard_dir = Direction::Down;
                    guard_x = x;
                    guard_y = y;
                }
                '<' => {
                    guard_dir = Direction::Left;
                    guard_x = x;
                    guard_y = y;
                }
                _ => panic!("unsupported character: {c}"),
            }

            x += 1;
        }

        y += 1;
    }

    MapInfo {
        max: Position { x: max_x, y: max_y },
        guard_pos: Position {
            x: guard_x,
            y: guard_y,
        },
        guard_dir,
        obstructions,
    }
}
