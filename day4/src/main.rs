mod input;

use input::*;

fn main() {
    part1();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;

    // horizontal
    for line in &lines {
        for i in 0..line.len() - 3 {
            // left to right
            if is_xmas(line[i], line[i + 1], line[i + 2], line[i + 3]) {
                result += 1;
            }
            // right to left
            if is_xmas(line[i + 3], line[i + 2], line[i + 1], line[i]) {
                result += 1;
            }
        }
    }

    // vertical
    for y in 0..lines.len() - 3 {
        for x in 0..lines[0].len() {
            // up to down
            if is_xmas(
                lines[y][x],
                lines[y + 1][x],
                lines[y + 2][x],
                lines[y + 3][x],
            ) {
                result += 1;
            }
            // down to up
            if is_xmas(
                lines[y + 3][x],
                lines[y + 2][x],
                lines[y + 1][x],
                lines[y][x],
            ) {
                result += 1;
            }
        }
    }

    // diagonals
    for y in 0..lines.len() - 3 {
        for x in 0..lines[0].len() - 3 {
            // down right
            if is_xmas(
                lines[y][x],
                lines[y + 1][x + 1],
                lines[y + 2][x + 2],
                lines[y + 3][x + 3],
            ) {
                result += 1;
            }
            // down left
            if is_xmas(
                lines[y][x + 3],
                lines[y + 1][x + 2],
                lines[y + 2][x + 1],
                lines[y + 3][x],
            ) {
                result += 1;
            }
            // up right
            if is_xmas(
                lines[y + 3][x],
                lines[y + 2][x + 1],
                lines[y + 1][x + 2],
                lines[y][x + 3],
            ) {
                result += 1;
            }
            // up left
            if is_xmas(
                lines[y + 3][x + 3],
                lines[y + 2][x + 2],
                lines[y + 1][x + 1],
                lines[y][x],
            ) {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn is_xmas(x: char, m: char, a: char, s: char) -> bool {
    x == 'X' && m == 'M' && a == 'A' && s == 'S'
}
