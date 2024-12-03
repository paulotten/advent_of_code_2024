mod input;

use input::*;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let mut result = 0;

    let input = get_input();
    let muls = parse_input1(input);

    for mul in muls {
        result += mul.0 * mul.1;
    }

    println!("{result}");
}

fn parse_input1(input: &str) -> Vec<(i32, i32)> {
    let mut results = vec![];

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    results
}

fn part2() {
    println!("Part 2");

    let mut result = 0;

    let input = get_input();
    let muls = parse_input2(input);

    for mul in muls {
        result += mul.0 * mul.1;
    }

    println!("{result}");
}

fn parse_input2(input: &str) -> Vec<(i32, i32)> {
    let mut results = vec![];
    let mut enabled = true;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re_mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    for captures in re.captures_iter(input) {
        let capture = captures.get(0).unwrap().as_str();

        match capture {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let sub_caps = re_mul.captures(capture).unwrap();

                    results.push((sub_caps[1].parse().unwrap(), sub_caps[2].parse().unwrap()));
                }
            }
        }
    }

    results
}
