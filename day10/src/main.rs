mod input;

use std::collections::HashSet;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let top_map = parse_topographic_map(input);

    let trailheads = get_trailheads(&top_map);
    let mut total_score = 0;

    for trailhead in &trailheads {
        let trailends = get_trailends(&top_map, trailhead);

        //println!("start {trailhead:?}");
        //println!("ends {trailends:?}");
        total_score += trailends.len();
    }

    println!("{total_score}");
}

fn part2() {
    println!("Part 1");

    let input = get_input();
    let top_map = parse_topographic_map(input);

    let trailheads = get_trailheads(&top_map);
    let mut total_rating = 0;

    for trailhead in &trailheads {
        let trails = get_trails(&top_map, trailhead);

        //println!("start {trailhead:?}");
        //println!("ends {trailends:?}");
        total_rating += trails.len();
    }

    println!("{total_rating}");
}

fn parse_topographic_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn get_trailheads(top_map: &Vec<Vec<u8>>) -> Vec<Position> {
    let mut trailheads = vec![];

    for y in 0..top_map.len() {
        for x in 0..top_map[y].len() {
            if top_map[y][x] == 0 {
                trailheads.push(Position { x, y });
            }
        }
    }

    trailheads
}

fn get_trailends(top_map: &Vec<Vec<u8>>, trailhead: &Position) -> HashSet<Position> {
    let mut working_set = HashSet::new();
    working_set.insert(trailhead.clone());

    // breadth first search
    for level in 1..=9 {
        let mut next_set = HashSet::new();

        for position in working_set {
            for adjacent in get_adjacent_positions(top_map, &position) {
                if top_map[adjacent.y][adjacent.x] == level {
                    next_set.insert(adjacent);
                }
            }
        }

        working_set = next_set;
        //println!("level {level} {working_set:?}");
    }

    working_set
}

fn get_trails(top_map: &Vec<Vec<u8>>, trailhead: &Position) -> Vec<Vec<Position>> {
    let mut working_set = vec![vec![trailhead.clone()]];

    // breadth first search
    for level in 1..=9 {
        let mut next_set = vec![];

        for trail in working_set {
            let position = trail.last().unwrap();

            for adjacent in get_adjacent_positions(top_map, &position) {
                if top_map[adjacent.y][adjacent.x] == level {
                    let mut new_trail = trail.clone();
                    new_trail.push(adjacent);
                    next_set.push(new_trail);
                }
            }
        }

        working_set = next_set;
        //println!("level {level} {working_set:?}");
    }

    working_set
}

fn get_adjacent_positions(top_map: &Vec<Vec<u8>>, position: &Position) -> Vec<Position> {
    let mut adjacent = vec![];

    if position.x > 0 {
        adjacent.push(Position {
            x: position.x - 1,
            y: position.y,
        });
    }
    if position.y > 0 {
        adjacent.push(Position {
            x: position.x,
            y: position.y - 1,
        });
    }
    if position.x < top_map[position.y].len() - 1 {
        adjacent.push(Position {
            x: position.x + 1,
            y: position.y,
        });
    }
    if position.y < top_map.len() - 1 {
        adjacent.push(Position {
            x: position.x,
            y: position.y + 1,
        });
    }

    adjacent
}
