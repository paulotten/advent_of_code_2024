mod input;

use std::collections::HashSet;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    /*
    let input = _get_sample_input();
    let max_coord = 6;
    let byte_count = 12;
    */

    //
    let input = get_input();
    let max_coord = 70;
    let byte_count = 1024;
    //

    // build memory
    let mut memory = vec![vec![true; max_coord + 1]; max_coord + 1];

    let mut lines = input.lines();

    for _ in 0..byte_count {
        let line = lines.next().unwrap();
        let mut split = line.split(',');

        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();

        memory[y][x] = false;
    }

    //_print_memory(&memory);

    // find shortest path
    let mut working_set = HashSet::new();
    working_set.insert(Position { x: 0, y: 0 });

    let mut visited = HashSet::new();
    let mut steps = 0;

    while !working_set.is_empty() {
        let mut new_set = HashSet::new();

        for pos in working_set {
            if pos.x as usize == max_coord && pos.y as usize == max_coord {
                println!("{steps}");
                return;
            }

            for new_pos in [
                Position {
                    x: pos.x - 1,
                    y: pos.y,
                },
                Position {
                    x: pos.x + 1,
                    y: pos.y,
                },
                Position {
                    x: pos.x,
                    y: pos.y - 1,
                },
                Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
            ] {
                if valid_position(&new_pos, max_coord, &memory) && !visited.contains(&new_pos) {
                    new_set.insert(new_pos);
                    visited.insert(new_pos);
                }
            }
        }

        working_set = new_set;
        steps += 1;
    }
}

fn part2() {
    println!("Part 2");

    /*
    let input = _get_sample_input();
    let max_coord = 6;
    */

    //
    let input = get_input();
    let max_coord = 70;
    //

    // build memory
    let mut memory = vec![vec![true; max_coord + 1]; max_coord + 1];

    let mut lines = input.lines();

    loop {
        let line = lines.next().unwrap();
        let mut split = line.split(',');

        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();

        memory[y][x] = false;

        if !can_exit(&memory, max_coord) {
            println!("{x},{y}");
            return;
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i8,
    y: i8,
}

fn _print_memory(memory: &Vec<Vec<bool>>) {
    for row in memory {
        for col in row {
            match col {
                true => print!("."),
                false => print!("#"),
            }
        }

        println!()
    }
    println!()
}

fn valid_position(pos: &Position, max_coord: usize, memory: &Vec<Vec<bool>>) -> bool {
    pos.x >= 0
        && pos.x as usize <= max_coord
        && pos.y >= 0
        && pos.y as usize <= max_coord
        && memory[pos.y as usize][pos.x as usize]
}

fn can_exit(memory: &Vec<Vec<bool>>, max_coord: usize) -> bool {
    let mut working_set = HashSet::new();
    working_set.insert(Position { x: 0, y: 0 });

    let mut visited = HashSet::new();

    while !working_set.is_empty() {
        let mut new_set = HashSet::new();

        for pos in working_set {
            if pos.x as usize == max_coord && pos.y as usize == max_coord {
                return true;
            }

            for new_pos in [
                Position {
                    x: pos.x - 1,
                    y: pos.y,
                },
                Position {
                    x: pos.x + 1,
                    y: pos.y,
                },
                Position {
                    x: pos.x,
                    y: pos.y - 1,
                },
                Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
            ] {
                if valid_position(&new_pos, max_coord, &memory) && !visited.contains(&new_pos) {
                    new_set.insert(new_pos);
                    visited.insert(new_pos);
                }
            }
        }

        working_set = new_set;
    }

    return false;
}
