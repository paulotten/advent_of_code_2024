mod input;

use std::collections::HashSet;

use input::*;

fn main() {
    part1();
}

fn part1() {
    println!("Part 1");

    //let input = _get_sample_input();
    let input = get_input();
    let plots = parse_input(input);

    let mut regions: Vec<Region> = vec![];
    let mut assigned_plots: HashSet<Position> = HashSet::new();

    let mut cur_region = Region {
        plant: plots[0][0],
        plots: HashSet::new(),
    };

    for y in 0..plots.len() {
        for x in 0..plots[y].len() {
            let pos = Position { x, y };

            let plant = plots[y][x];

            if plant != cur_region.plant {
                if !cur_region.plots.is_empty() {
                    regions.push(cur_region);
                }

                cur_region = Region {
                    plant,
                    plots: HashSet::new(),
                };
            }
            
            if assigned_plots.contains(&pos) {
                continue;
            }

            insert_recursive(&plots, &mut cur_region, &mut assigned_plots, &pos);
        }
    }
    regions.push(cur_region);

    let mut total_price = 0;

    for region in regions {
        let area = region.plots.len();
        let perimeter = get_perimeter(&plots, &region);
        let price = area * perimeter;

        println!("A region of '{}' plants with price {area} * {perimeter} = {price}", region.plant);

        total_price += price;
    }

    println!("Total price = {total_price}");
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect()
}

fn get_adjacent(plots: &Vec<Vec<char>>, pos: &Position) -> Vec<Position> {
    let mut adjacent = vec![];

    if pos.x > 0 {
        adjacent.push(Position {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.x < plots[0].len() - 1 {
        adjacent.push(Position {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        adjacent.push(Position {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.y < plots.len() - 1 {
        adjacent.push(Position {
            x: pos.x,
            y: pos.y + 1,
        });
    }

    adjacent
}

fn insert_recursive(plots: &Vec<Vec<char>>, cur_region: &mut Region, assigned_plots: &mut HashSet<Position>, pos: &Position) {
    cur_region.plots.insert(pos.clone());
    assigned_plots.insert(pos.clone());

    for pos in get_adjacent(&plots, pos) {
        if assigned_plots.contains(&pos) {
            continue;
        }

        let plant = plots[pos.y][pos.x];

        if plant == cur_region.plant {
            insert_recursive(plots, cur_region, assigned_plots, &pos);
        }
    }
}

fn get_perimeter(plots: &Vec<Vec<char>>, region: &Region) -> usize {
    let mut perimeter = region.plots.len() * 4;

    for pos in &region.plots {
        for adj in get_adjacent(plots, pos) {
            if region.plots.contains(&adj) {
                perimeter -= 1;
            }
        }
    }

    perimeter
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Region {
    plant: char,
    plots: HashSet<Position>,
}
