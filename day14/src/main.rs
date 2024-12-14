mod input;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    //let input = _get_sample_input();
    //let (bound_x, bound_y) = _get_sample_bounds();
    let input = get_input();
    let (bound_x, bound_y) = get_bounds();

    let mut robots = parse_input(input);

    // simulate 100 seconds
    for _second in 1..=100 {
        for r in &mut robots {
            r.pos_x = bound(r.pos_x + r.vel_x, bound_x);
            r.pos_y = bound(r.pos_y + r.vel_y, bound_y);
        }
    }

    // tally robots by quadrant
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for r in &robots {
        if r.pos_x < bound_x / 2 {
            if r.pos_y < bound_y / 2 {
                q1 += 1;
            } else if r.pos_y > bound_y / 2 {
                q3 += 1;
            }
        } else if r.pos_x > bound_x / 2 {
            if r.pos_y < bound_y / 2 {
                q2 += 1;
            } else if r.pos_y > bound_y / 2 {
                q4 += 1;
            }
        }
    }

    //println!("{q1}, {q2}, {q3}, {q4}");

    let result = q1 * q2 * q3 * q4;
    println!("{result}");
}

// strategy: dump output to a file, scroll through output manually looking for a Christmas tree.
fn part2() {
    println!("Part 2");

    let input = get_input();
    let (bound_x, bound_y) = get_bounds();

    let mut robots = parse_input(input);
    let mut second = 0;

    loop {
        second += 1;

        for r in &mut robots {
            r.pos_x = bound(r.pos_x + r.vel_x, bound_x);
            r.pos_y = bound(r.pos_y + r.vel_y, bound_y);
        }

        if maybe_christmas_tree(&robots, bound_x, bound_y) {
            println!("second {second}");
            print_robots(&robots, bound_x, bound_y);
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = vec![];

    for line in input.lines() {
        let pv: Vec<_> = line.split(' ').collect();
        assert!(pv.len() == 2);

        let p: Vec<_> = pv[0].split("p=").collect();
        assert!(p.len() == 2);
        let p: Vec<_> = p[1].split(",").collect();
        assert!(p.len() == 2);

        let v: Vec<_> = pv[1].split("v=").collect();
        assert!(v.len() == 2);
        let v: Vec<_> = v[1].split(",").collect();
        assert!(v.len() == 2);

        robots.push(Robot {
            pos_x: p[0].parse().unwrap(),
            pos_y: p[1].parse().unwrap(),
            vel_x: v[0].parse().unwrap(),
            vel_y: v[1].parse().unwrap(),
        });
    }

    robots
}

fn bound(num: i32, limit: i32) -> i32 {
    let mut num = num % limit;

    if num < 0 {
        num += limit;
    }

    num
}

fn print_robots(robots: &Vec<Robot>, bound_x: i32, bound_y: i32) {
    let mut map = vec![];

    // build a map
    for _y in 0..bound_y {
        let mut line = vec![];

        for _x in 0..bound_x {
            line.push(0);
        }

        map.push(line);
    }

    // add robots to map
    for r in robots {
        map[r.pos_y as usize][r.pos_x as usize] += 1;
    }

    // print map
    for line in map {
        for count in line {
            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
        }
        println!();
    }
}

fn maybe_christmas_tree(robots: &Vec<Robot>, _bound_x: i32, bound_y: i32) -> bool {
    // I'm thinking a Christmas tree is basically a triangle,
    // that should have more volume on the bottom than the top

    // After completing the challenge it looks like the Christmas tree is in one particular quadrant.
    // So probably the "correct" solution is to look for one quadrant with an above average amount of robots.
    // The instructions gave no hint about this though and I like my solution, so I'm keeping it.

    let mut top_vol = 0;
    let mut bot_vol = 0;

    for r in robots {
        if r.pos_y < bound_y / 2 {
            top_vol += 1;
        } else if r.pos_y > bound_y / 2 {
            bot_vol += 1;
        }
    }

    top_vol < bot_vol / 2
}

#[derive(Debug)]
struct Robot {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}
