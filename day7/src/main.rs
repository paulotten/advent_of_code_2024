mod input;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let parsed = parse_input(input);

    let mut result = 0;

    for (target, values) in parsed {
        let mut working_set = vec![values[0]];

        for i in 1..values.len() {
            let mut next_set = vec![];

            for work in working_set {
                next_set.push(work+values[i]);
                next_set.push(work*values[i]);
            }

            working_set = next_set;
        }

        if working_set.contains(&target) {
            result += target;
        }
    }

    println!("{result}");
}

fn part2() {
    println!("Part 2");

    let input = get_input();
    let parsed = parse_input(input);

    let mut result = 0;

    for (target, values) in parsed {
        let mut working_set = vec![values[0]];

        for i in 1..values.len() {
            let mut next_set = vec![];

            for work in working_set {
                let concat: i64 = format!("{work}{}", values[i]).parse().unwrap();
                next_set.push(concat);
                next_set.push(work+values[i]);
                next_set.push(work*values[i]);
            }

            working_set = next_set;
        }

        if working_set.contains(&target) {
            result += target;
        }
    }

    println!("{result}");
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    let mut parsed = vec![];

    for line in input.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        assert!(parts.len() == 2);

        let target: i64 = parts[0].parse().unwrap();
        let values: Vec<i64> = parts[1].split(' ').map(|v| v.parse().unwrap()).collect();

        parsed.push((target, values));
    }

    parsed
}
