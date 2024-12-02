mod input;

use std::collections::HashMap;
use input::*;

fn main() {
    part1();
    part2();
}

// solution for part 1
fn part1() {
    println!("Part 1");

    // track result value, which we add to as we go
    let mut result = 0;

    // get left and right vectors from input
    let (mut left, mut right) = parse_input(&get_input());

    // sort left and right vectors
    left.sort();
    right.sort();

    // then calculate the absolute difference between each number from each list
    for i in 0..left.len() {
        let diff = (left[i] - right[i]).abs();

        // add abs the result
        result += diff;
    }

    println!("{}", result);
}

// solution for part 2
fn part2() {
    println!("Part 2");

    // track result value, which we add to as we go
    let mut result = 0;

    // get left and right vectors from input
    let (left, right) = parse_input(&get_input());

    // create a hashmap to track occurances of values in the right vector
    // goal is order-n performance
    let mut map: HashMap<i32, i32> = HashMap::new();

    // count the number of occurances of values in the right vector
    for right_key in right {
        map.entry(right_key)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // then for each value in the left vector
    for left_key in left {
        // check how many times it occurs in the right list
        if map.contains_key(&left_key) {
            // then we add the number of occurances * `left_key`` to `result``
            result += left_key * map.get(&left_key).unwrap();
        }
    }

    println!("{}", result);
}

// parse the input into a left and a right vector
fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    // for each line of input
    for line in input.lines() {
        // split into a left and a right part
        let parts: Vec<_> = line.split("   ").collect();

        // make sure we have exactly 2 parts
        assert!(parts.len() == 2);

        // parse left and right parts to integers
        // then add to left and right vectors
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    // return left and right vectors
    (left, right)
}
