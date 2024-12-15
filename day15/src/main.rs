mod input;

use std::collections::{BTreeSet, HashMap};

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    //let input = _get_sample_input_small();
    //let input = _get_sample_input_large();
    let input = get_input();
    let (mut warehouse, mut robot, moves) = parse_input(input);

    //println!("Initial state:");
    //_print_warehouse(&warehouse, &robot);

    for m in moves.chars() {
        match m {
            '<' => try_left(&mut warehouse, &mut robot),
            '>' => try_right(&mut warehouse, &mut robot),
            '^' => try_up(&mut warehouse, &mut robot),
            'v' => try_down(&mut warehouse, &mut robot),
            _ => panic!("Unsupported move: {m}"),
        };

        //println!("Move {m}:");
        //_print_warehouse(&warehouse, &robot);
    }

    //println!("Final state:");
    //_print_warehouse(&warehouse, &robot);

    println!("score: {}", get_score(&warehouse));
}

fn part2() {
    println!("Part 2");

    //let input = &_get_sample_input2();
    //let input = _get_sample_input_large();
    let input = get_input();
    let (warehouse, mut robot, moves) = parse_input(input);

    let mut warehouse = widen_warehouse(&warehouse);
    robot.x *= 2;

    //println!("Initial state:");
    //_print_warehouse2(&warehouse, &robot);

    for m in moves.chars() {
        //println!("Move {m}:");
        match m {
            '<' => try_left2(&mut warehouse, &mut robot),
            '>' => try_right2(&mut warehouse, &mut robot),
            '^' => try_up2(&mut warehouse, &mut robot),
            'v' => try_down2(&mut warehouse, &mut robot),
            _ => panic!("Unsupported move: {m}"),
        };
        //_print_warehouse2(&warehouse, &robot);
    }

    //println!("Final state:");
    //_print_warehouse2(&warehouse, &robot);

    println!("score: {}", get_score2(&warehouse));
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile2 {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Position, String) {
    let mut warehouse = vec![];
    let mut robot = None;
    let mut moves = String::new();

    let mut y = 0;
    for line in input.lines() {
        if line.starts_with('#') {
            let mut row = vec![];

            let mut x = 0;
            for c in line.chars() {
                let tile = match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '.' => Tile::Empty,
                    '@' => {
                        robot = Some(Position { x, y });
                        Tile::Empty
                    }
                    _ => panic!("Unsupported tile: {c}"),
                };

                row.push(tile);
                x += 1;
            }

            warehouse.push(row);
            y += 1;
        } else if !line.is_empty() {
            moves += line;
        }
    }

    (warehouse, robot.unwrap(), moves)
}

fn try_left(warehouse: &mut Vec<Vec<Tile>>, robot: &mut Position) {
    let mut x = robot.x - 1;
    let y = robot.y;

    while x >= 0 {
        match warehouse[y as usize][x as usize] {
            Tile::Wall => return,
            Tile::Box => {}
            Tile::Empty => {
                robot.x -= 1;

                // push boxes
                if warehouse[robot.y as usize][robot.x as usize] == Tile::Box {
                    warehouse[y as usize][x as usize] = Tile::Box;
                    warehouse[robot.y as usize][robot.x as usize] = Tile::Empty;
                }

                return;
            }
        }

        x -= 1;
    }
}

fn try_left2(warehouse: &mut Vec<Vec<Tile2>>, robot: &mut Position) {
    let mut x = robot.x - 1;
    let y = robot.y;

    while x >= 0 {
        match warehouse[y as usize][x as usize] {
            Tile2::Wall => return,
            Tile2::BoxLeft | Tile2::BoxRight => {}
            Tile2::Empty => {
                robot.x -= 1;

                // push boxes
                for i in x..=robot.x {
                    warehouse[y as usize][i as usize] = warehouse[y as usize][(i+1) as usize];
                }

                return;
            }
        }

        x -= 1;
    }
}

fn try_right(warehouse: &mut Vec<Vec<Tile>>, robot: &mut Position) {
    let mut x = robot.x + 1;
    let y = robot.y;

    while (x as usize) < warehouse[y as usize].len() {
        match warehouse[y as usize][x as usize] {
            Tile::Wall => return,
            Tile::Box => {}
            Tile::Empty => {
                robot.x += 1;

                // push boxes
                if warehouse[robot.y as usize][robot.x as usize] == Tile::Box {
                    warehouse[y as usize][x as usize] = Tile::Box;
                    warehouse[robot.y as usize][robot.x as usize] = Tile::Empty;
                }

                return;
            }
        }

        x += 1;
    }
}

fn try_right2(warehouse: &mut Vec<Vec<Tile2>>, robot: &mut Position) {
    let mut x = robot.x + 1;
    let y = robot.y;

    while (x as usize) < warehouse[y as usize].len() {
        match warehouse[y as usize][x as usize] {
            Tile2::Wall => return,
            Tile2::BoxLeft | Tile2::BoxRight => {}
            Tile2::Empty => {
                robot.x += 1;

                // push boxes
                for i in (robot.x..=x).rev() {
                    warehouse[y as usize][i as usize] = warehouse[y as usize][(i-1) as usize];
                }

                return;
            }
        }

        x += 1;
    }
}

fn try_up(warehouse: &mut Vec<Vec<Tile>>, robot: &mut Position) {
    let x = robot.x;
    let mut y = robot.y - 1;

    while y >= 0 {
        match warehouse[y as usize][x as usize] {
            Tile::Wall => return,
            Tile::Box => {}
            Tile::Empty => {
                robot.y -= 1;

                // push boxes
                if warehouse[robot.y as usize][robot.x as usize] == Tile::Box {
                    warehouse[y as usize][x as usize] = Tile::Box;
                    warehouse[robot.y as usize][robot.x as usize] = Tile::Empty;
                }

                return;
            }
        }

        y -= 1;
    }
}

fn try_up2(warehouse: &mut Vec<Vec<Tile2>>, robot: &mut Position) {
    let mut x_min = robot.x;
    let mut x_max = robot.x;
    let mut y = robot.y - 1;

    let mut blocks_pushing: HashMap<i32 /*y*/, BTreeSet<i32 /*x*/>> = HashMap::new();

    while y >= 0 {
        let mut set = BTreeSet::new();

        for x in x_min..=x_max {
            match warehouse[y as usize][x as usize] {
                Tile2::Wall => return,
                Tile2::BoxLeft => {
                    set.insert(x);
                    set.insert(x+1);
                }
                Tile2::BoxRight => {
                    set.insert(x);
                    set.insert(x-1);
                }
                Tile2::Empty => {}
            }
        }

        if set.is_empty() {
            robot.y -= 1;

            // push boxes
            if !blocks_pushing.is_empty() {
                //println!("{blocks_pushing:?}");

                for y in y+1..=robot.y {
                    //println!("{y}");
                    //println!("{:?}", blocks_pushing.get(&y).unwrap());

                    for x in blocks_pushing.get(&y).unwrap() {
                        let x = *x;
                        warehouse[y as usize -1][x as usize] = warehouse[y as usize][x as usize];
                        warehouse[y as usize][x as usize] = Tile2::Empty;
                    }
                }
            }

            return;
        } else {
            x_min = *set.first().unwrap();
            x_max = *set.last().unwrap();

            blocks_pushing.insert(y, set);
        }

        y -= 1;
    }
}

fn try_down(warehouse: &mut Vec<Vec<Tile>>, robot: &mut Position) {
    let x = robot.x;
    let mut y = robot.y + 1;

    while (y as usize) < warehouse.len() {
        match warehouse[y as usize][x as usize] {
            Tile::Wall => return,
            Tile::Box => {}
            Tile::Empty => {
                robot.y += 1;

                // push boxes
                if warehouse[robot.y as usize][robot.x as usize] == Tile::Box {
                    warehouse[y as usize][x as usize] = Tile::Box;
                    warehouse[robot.y as usize][robot.x as usize] = Tile::Empty;
                }

                return;
            }
        }

        y += 1;
    }
}


fn try_down2(warehouse: &mut Vec<Vec<Tile2>>, robot: &mut Position) {
    let mut x_min = robot.x;
    let mut x_max = robot.x;
    let mut y = robot.y + 1;

    let mut blocks_pushing: HashMap<i32 /*y*/, BTreeSet<i32 /*x*/>> = HashMap::new();

    while (y as usize) < warehouse.len() {
        let mut set = BTreeSet::new();

        for x in x_min..=x_max {
            match warehouse[y as usize][x as usize] {
                Tile2::Wall => return,
                Tile2::BoxLeft => {
                    set.insert(x);
                    set.insert(x+1);
                }
                Tile2::BoxRight => {
                    set.insert(x);
                    set.insert(x-1);
                }
                Tile2::Empty => {}
            }
        }

        if set.is_empty() {
            robot.y += 1;

            // push boxes
            if !blocks_pushing.is_empty() {
                //println!("{blocks_pushing:?}");

                for y in (robot.y..=y-1).rev() {
                    //println!("{y}");
                    //println!("{:?}", blocks_pushing.get(&y).unwrap());

                    for x in blocks_pushing.get(&y).unwrap() {
                        let x = *x;
                        warehouse[y as usize +1][x as usize] = warehouse[y as usize][x as usize];
                        warehouse[y as usize][x as usize] = Tile2::Empty;
                    }
                }
            }

            return;
        } else {
            x_min = *set.first().unwrap();
            x_max = *set.last().unwrap();

            blocks_pushing.insert(y, set);
        }

        y += 1;
    }
}

fn _print_warehouse(warehouse: &Vec<Vec<Tile>>, robot: &Position) {
    let mut y = 0;
    for row in warehouse {
        let mut x = 0;
        for tile in row {
            match tile {
                Tile::Wall => print!("#"),
                Tile::Box => print!("O"),
                Tile::Empty => {
                    if robot.x == x && robot.y == y {
                        print!("@");
                    } else {
                        print!(".");
                    }
                },
            }
            x += 1;
        }
        println!();
        y += 1;
    }
    println!();
}

fn _print_warehouse2(warehouse: &Vec<Vec<Tile2>>, robot: &Position) {
    let mut y = 0;
    for row in warehouse {
        let mut x = 0;
        for tile in row {
            match tile {
                Tile2::Wall => print!("#"),
                Tile2::BoxLeft => print!("["),
                Tile2::BoxRight => print!("]"),
                Tile2::Empty => {
                    if robot.x == x && robot.y == y {
                        print!("@");
                    } else {
                        print!(".");
                    }
                },
            }
            x += 1;
        }
        println!();
        y += 1;
    }
    println!();
}

fn widen_warehouse(warehouse: &Vec<Vec<Tile>>) -> Vec<Vec<Tile2>> {
    let mut wide = vec![];

    for row in warehouse {
        let mut wide_row = vec![];

        for tile in row {
            match tile {
                Tile::Wall => {
                    wide_row.push(Tile2::Wall);
                    wide_row.push(Tile2::Wall);
                },
                Tile::Box => {
                    wide_row.push(Tile2::BoxLeft);
                    wide_row.push(Tile2::BoxRight);
                },
                Tile::Empty => {
                    wide_row.push(Tile2::Empty);
                    wide_row.push(Tile2::Empty);
                },
            }
        }

        wide.push(wide_row);
    }

    wide
}

fn get_score(warehouse: &Vec<Vec<Tile>>) -> i32 {
    let mut sum = 0;

    let mut y = 0;
    for row in warehouse {
        let mut x = 0;
        for tile in row {
            if *tile == Tile::Box {
                sum += y*100 + x;
            }

            x += 1;
        }
        y += 1;
    }

    sum
}

fn get_score2(warehouse: &Vec<Vec<Tile2>>) -> i32 {
    let mut sum = 0;

    let mut y = 0;
    for row in warehouse {
        let mut x = 0;
        for tile in row {
            if *tile == Tile2::BoxLeft {
                sum += y*100 + x;
            }

            x += 1;
        }
        y += 1;
    }

    sum
}
