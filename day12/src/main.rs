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
    let plots = parse_input(input);
    let regions = get_regions(&plots);

    let mut total_price = 0;

    for region in regions {
        let area = region.plots.len();
        let perimeter = get_perimeter(&plots, &region);
        let price = area * perimeter;

        //println!("A region of '{}' plants with price {area} * {perimeter} = {price}", region.plant);

        total_price += price;
    }

    println!("Total price = {total_price}");
}

fn part2() {
    println!("Part 2");

    //let input = _get_sample_input2();
    let input = get_input();
    let plots = parse_input(input);
    let regions = get_regions(&plots);

    let mut total_price = 0;

    for region in regions {
        let area = region.plots.len();
        let sides = get_sides(&region);
        let price = area * sides;

        /*
        println!(
            "A region of '{}' plants with price {area} * {sides} = {price}",
            region.plant
        );
        */

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

fn get_regions(plots: &Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut assigned_plots: HashSet<Position> = HashSet::new();

    for y in 0..plots.len() {
        for x in 0..plots[y].len() {
            let pos = Position { x, y };

            if assigned_plots.contains(&pos) {
                continue;
            }

            let plant = plots[y][x];

            let mut cur_region = Region {
                plant,
                plots: HashSet::new(),
            };

            insert_recursive(&plots, &mut cur_region, &mut assigned_plots, &pos);
            regions.push(cur_region);
        }
    }

    regions
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

fn insert_recursive(
    plots: &Vec<Vec<char>>,
    cur_region: &mut Region,
    assigned_plots: &mut HashSet<Position>,
    pos: &Position,
) {
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

fn get_sides(region: &Region) -> usize {
    let plots = plot_region(region);
    let holes = get_holes(&plots);

    //_print_plots(&plots);

    let mut sides = 4;

    for hole in holes {
        let mut hole_sides = get_sides(&hole);

        // - 2 if the hole overlaps a corner
        if hole.plots.contains(&Position { x: 0, y: 0 })
            || hole.plots.contains(&Position {
                x: 0,
                y: plots.len() - 1,
            })
            || hole.plots.contains(&Position {
                x: plots[0].len() - 1,
                y: 0,
            })
            || hole.plots.contains(&Position {
                x: plots[0].len() - 1,
                y: plots.len() - 1,
            })
        {
            hole_sides -= 2;
        }

        sides += hole_sides
    }

    sides
}

fn _print_plots(plots: &Vec<Vec<char>>) {
    for line in plots {
        for c in line {
            print!("{c}");
        }

        println!();
    }
    println!();
}

fn get_holes(plots: &Vec<Vec<char>>) -> Vec<Region> {
    let regions = get_regions(plots);

    regions.into_iter().filter(|r| r.plant == '.').collect()
}

fn plot_region(region: &Region) -> Vec<Vec<char>> {
    let mut plots = vec![];

    let mut iter = region.plots.iter();
    let first = iter.next().unwrap();
    let mut min_x = first.x;
    let mut min_y = first.y;
    let mut max_x = first.x;
    let mut max_y = first.y;

    for pos in iter {
        if pos.x < min_x {
            min_x = pos.x;
        }
        if pos.y < min_y {
            min_y = pos.y
        }
        if pos.x > max_x {
            max_x = pos.x;
        }
        if pos.y > max_y {
            max_y = pos.y
        }
    }

    for y in min_y..=max_y {
        let mut line = vec![];

        for x in min_x..=max_x {
            let pos = Position { x, y };

            if region.plots.contains(&pos) {
                line.push('#');
            } else {
                line.push('.');
            }
        }

        plots.push(line);
    }

    plots
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
