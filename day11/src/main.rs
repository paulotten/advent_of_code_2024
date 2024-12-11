mod input;

use std::collections::HashMap;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    let input = get_input();
    let mut stones = parse_input(input);

    //println!("{stones:?}");

    for _i in 1..=25 {
        stones = blink(&stones);

        /*
        println!();
        println!("After {_i} blinks:");
        println!("{stones:?}")
        */
    }

    println!("{}", stones.len());
}

fn part2() {
    println!("Part 2");

    let input = get_input();
    let stones = parse_input(input);
    let mut stones = map_stones(stones);

    for _i in 1..=75 {
        stones = blink_map(stones);

        println!("After {_i} blinks: {}", stones.values().sum::<u64>());
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split(' ').map(|i| i.parse().unwrap()).collect()
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = vec![];

    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
        } else if is_even_digits(*stone) {
            let split_stones = split_stone(*stone);
            
            new_stones.push(split_stones.0);
            new_stones.push(split_stones.1);
        } else {
            new_stones.push(*stone * 2024);
        }
    }

    new_stones
}

fn map_stones(stones: Vec<u64>) -> HashMap<u64, u64> {
    let mut stones_map = HashMap::new();

    for stone in stones {
        stones_map.entry(stone).and_modify(|c| *c += 1).or_insert(1);
    }

    stones_map
}

fn blink_map(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();

    for (stone, count) in stones {
        if stone == 0 {
            new_stones.entry(1).and_modify(|c| *c += count).or_insert(count);
        } else if is_even_digits(stone) {
            let split_stones = split_stone(stone);
            
            new_stones.entry(split_stones.0).and_modify(|c| *c += count).or_insert(count);
            new_stones.entry(split_stones.1).and_modify(|c| *c += count).or_insert(count);
        } else {
            new_stones.entry(stone * 2024).and_modify(|c| *c += count).or_insert(count);
        }
    }

    new_stones
}

fn is_even_digits(stone: u64) -> bool {
    let s = format!("{stone}");

    s.len() % 2 == 0
}

fn split_stone(stone: u64) -> (u64, u64) {
    let s = format!("{stone}");
    let half_len = s.len()/2;

    let a = s[0..half_len].parse().unwrap();
    let b = s[half_len..].parse().unwrap();

    (a, b)
}
