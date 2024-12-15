mod input;

use input::*;

fn main() {
    part1();
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

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Box,
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
