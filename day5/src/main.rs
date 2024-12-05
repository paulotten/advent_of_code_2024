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
    let (rules, updates) = parse_input(input);
    let rules = map_rules(rules);

    let mut result = 0;

    for update in updates {
        let valid = is_valid_update(&rules, &update);

        if valid {
            result += update[update.len() / 2];
        }
    }

    println!("{result}");
}

fn part2() {
    println!("Part 2");

    let input = get_input();
    let (rules, updates) = parse_input(input);
    let rules = map_rules(rules);

    let mut result = 0;

    for mut update in updates {
        let valid = is_valid_update(&rules, &update);

        if !valid {
            reorder_update(&rules, &mut update);
            result += update[update.len() / 2];
        }
    }

    println!("{result}");
}

fn reorder_update(rules: &HashMap<i32, Vec<i32>>, update: &mut Vec<i32>) {
    for i in 0..update.len() {
        if rules.contains_key(&update[i]) {
            let rule = rules.get(&update[i]).unwrap();

            for j in 0..i {
                if rule.contains(&update[j]) {
                    let swap = update[j];
                    update[j] = update[i];
                    update[i] = swap;

                    return reorder_update(rules, update);
                }
            }
        }
    }
}

fn is_valid_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    for i in 0..update.len() {
        if rules.contains_key(&update[i]) {
            let rule = rules.get(&update[i]).unwrap();

            for j in 0..i {
                if rule.contains(&update[j]) {
                    return false;
                }
            }
        }
    }

    true
}

fn map_rules(rules: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut map = HashMap::new();

    for rule in rules {
        map.entry(rule.0)
            .and_modify(|v: &mut Vec<_>| v.push(rule.1))
            .or_insert(vec![rule.1]);
    }

    map
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = vec![];
    let mut updates = vec![];

    let mut mid_point = false;

    for line in input.lines() {
        if !mid_point {
            if line.is_empty() {
                mid_point = true;
            } else {
                let parts: Vec<_> = line.split('|').collect();

                assert!(parts.len() == 2);

                rules.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
            }
        } else {
            let update: Vec<_> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

            updates.push(update);
        }
    }

    (rules, updates)
}
