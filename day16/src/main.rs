mod input;

use std::collections::{BTreeMap, HashSet};

use input::*;

fn main() {
    parts_1_2();
}

fn parts_1_2() {
    //println!("Parts 1 and 2");

    //println!("{}", size_of::<Position>());

    //let input = _get_sample_input2();
    let input = get_input();
    let map = parse_input(input);

    let mut found_end = false;
    let mut paths_by_score: BTreeMap<i32, HashSet<Path>> = BTreeMap::new();

    let start_path = Path {
        pos: map.start,
        dir: Direction::East,
        // use imutable vectors to reduce memory usage when cloning
        history: im::vector![map.start],
    };
    let set = paths_by_score.entry(0).or_default();
    set.insert(start_path);

    let mut visited = HashSet::new();
    let mut best_path_pos = HashSet::new();

    while !paths_by_score.is_empty() && !found_end {
        let (score, set) = paths_by_score.pop_first().unwrap();

        let mut new_visited = HashSet::new();

        for path in set {
            if path.pos == map.end {
                if !found_end {
                    println!("Part 1");
                    println!("{score}");
                    found_end = true;
                }

                for pos in path.history.clone() {
                    best_path_pos.insert(pos);
                }
            }

            if visited.contains(&(path.pos, path.dir)) {
                continue;
            }
            new_visited.insert((path.pos, path.dir));

            let north = Position {
                x: path.pos.x,
                y: path.pos.y - 1,
            };
            let west = Position {
                x: path.pos.x - 1,
                y: path.pos.y,
            };
            let east = Position {
                x: path.pos.x + 1,
                y: path.pos.y,
            };
            let south = Position {
                x: path.pos.x,
                y: path.pos.y + 1,
            };

            match path.dir {
                Direction::North => {
                    if valid_space(&map.tiles, &north) {
                        let mut history = path.history.clone();
                        history.push_back(north);

                        let set = paths_by_score.entry(score + 1).or_default();
                        set.insert(Path {
                            pos: north,
                            dir: Direction::North,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &west) {
                        let mut history = path.history.clone();
                        history.push_back(west);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: west,
                            dir: Direction::West,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &east) {
                        let mut history = path.history.clone();
                        history.push_back(east);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: east,
                            dir: Direction::East,
                            history,
                        });
                    }
                }
                Direction::West => {
                    if valid_space(&map.tiles, &west) {
                        let mut history = path.history.clone();
                        history.push_back(west);

                        let set = paths_by_score.entry(score + 1).or_default();
                        set.insert(Path {
                            pos: west,
                            dir: Direction::West,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &north) {
                        let mut history = path.history.clone();
                        history.push_back(north);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: north,
                            dir: Direction::North,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &south) {
                        let mut history = path.history.clone();
                        history.push_back(south);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: south,
                            dir: Direction::South,
                            history,
                        });
                    }
                }
                Direction::East => {
                    if valid_space(&map.tiles, &east) {
                        let mut history = path.history.clone();
                        history.push_back(east);

                        let set = paths_by_score.entry(score + 1).or_default();
                        set.insert(Path {
                            pos: east,
                            dir: Direction::East,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &north) {
                        let mut history = path.history.clone();
                        history.push_back(north);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: north,
                            dir: Direction::North,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &south) {
                        let mut history = path.history.clone();
                        history.push_back(south);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: south,
                            dir: Direction::South,
                            history,
                        });
                    }
                }
                Direction::South => {
                    if valid_space(&map.tiles, &south) {
                        let mut history = path.history.clone();
                        history.push_back(south);

                        let set = paths_by_score.entry(score + 1).or_default();
                        set.insert(Path {
                            pos: south,
                            dir: Direction::South,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &west) {
                        let mut history = path.history.clone();
                        history.push_back(west);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: west,
                            dir: Direction::West,
                            history,
                        });
                    }

                    if valid_space(&map.tiles, &east) {
                        let mut history = path.history.clone();
                        history.push_back(east);

                        let set = paths_by_score.entry(score + 1001).or_default();
                        set.insert(Path {
                            pos: east,
                            dir: Direction::East,
                            history,
                        });
                    }
                }
            };
        }

        for pd in new_visited {
            visited.insert(pd);
        }

        //println!("{paths_by_score:?}");
    }

    println!("Part 2");
    println!("{}", best_path_pos.len());
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Space,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    // prefer u8 for lower memory usage
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct PuzzleMap {
    tiles: Vec<Vec<Tile>>,
    start: Position,
    end: Position,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Path {
    pos: Position,
    dir: Direction,
    history: im::Vector<Position>,
}

fn parse_input(input: &str) -> PuzzleMap {
    let mut tiles = vec![];
    let mut start = None;
    let mut end = None;

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        let mut row = vec![];

        for c in line.chars() {
            row.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Space,
                'E' => {
                    end = Some(Position { x, y });

                    Tile::Space
                }
                'S' => {
                    start = Some(Position { x, y });

                    Tile::Space
                }
                _ => panic!("Unsupported tile: {c}"),
            });

            x += 1;
        }

        tiles.push(row);
        y += 1;
    }

    PuzzleMap {
        tiles,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn valid_space(tiles: &Vec<Vec<Tile>>, pos: &Position) -> bool {
    let x = pos.x;
    let y = pos.y;

    if x as usize >= tiles[0].len() || y as usize >= tiles.len() {
        return false;
    }

    tiles[y as usize][x as usize] == Tile::Space
}
