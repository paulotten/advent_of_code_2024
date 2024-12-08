mod input;

use std::collections::{HashMap, HashSet};

use input::*;

fn main() {
    part1();
    part2();
}

/*
Note: I consider this solution incomplete.

If two antennas are three spaces apart then there should be (two) valid antinodes in between them,
one space from the one antenna and two spaces from the other antenna.
*/
fn part1() {
    println!("Part 1");

    let input = get_input();
    let MapInfo { max, antennas } = parse_input(input);

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_c, ants) in antennas {
        for i in 0..ants.len() {
            for j in i+1..ants.len() {
                let ix = ants[i].x;
                let iy = ants[i].y;
                let jx = ants[j].x;
                let jy = ants[j].y;

                let diff_x = jx - ix;
                let diff_y = jy - iy;

                let test_x = jx + diff_x;
                let test_y = jy + diff_y;

                if test_x >= 0 && test_x <= max.x && test_y >= 0 && test_y <= max.y {
                    antinodes.insert(Position {x: test_x, y: test_y});
                }

                let test_x = ix - diff_x;
                let test_y = iy - diff_y;

                if test_x >= 0 && test_x <= max.x && test_y >= 0 && test_y <= max.y {
                    antinodes.insert(Position {x: test_x, y: test_y});
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

/*
Note: I also consider this solution incomplete.

In addition to my concern from part 1,
if two antennas are two spaces apart then this algorithm will record an antinode every two spaces.
However every one space should also be a valid antinode as it is also perfectly in line with two antennas.
*/
fn part2() {
    println!("Part 2");

    let input = get_input();
    let MapInfo { max, antennas } = parse_input(input);

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_c, ants) in antennas {
        for i in 0..ants.len() {
            for j in i+1..ants.len() {
                let ix = ants[i].x;
                let iy = ants[i].y;
                let jx = ants[j].x;
                let jy = ants[j].y;

                let diff_x = jx - ix;
                let diff_y = jy - iy;

                let mut test_x = jx;
                let mut test_y = jy;

                while test_x >= 0 && test_x <= max.x && test_y >= 0 && test_y <= max.y {
                    antinodes.insert(Position {x: test_x, y: test_y});

                    test_x += diff_x;
                    test_y += diff_y;
                }

                let mut test_x = ix;
                let mut test_y = iy;

                //println!("c={_c} i={ix},{iy} j={jx},{jy} test={test_x},{test_y}");

                while test_x >= 0 && test_x <= max.x && test_y >= 0 && test_y <= max.y {
                    antinodes.insert(Position {x: test_x, y: test_y});

                    test_x -= diff_x;
                    test_y -= diff_y;
                }
            }
        }
    }

    //println!("{antinodes:?}");
    println!("{}", antinodes.len());
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct MapInfo {
    max: Position,
    antennas: HashMap<char, Vec<Position>>,
}

fn parse_input(input: &str) -> MapInfo {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;

        for c in line.chars() {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|v| v.push(Position { x, y }))
                    .or_insert(vec![Position { x, y }]);
            }

            x += 1;
        }

        y += 1;
    }

    let max_y = y - 1;
    let max_x = x - 1;

    MapInfo {
        max: Position { x: max_x, y: max_y },
        antennas,
    }
}
