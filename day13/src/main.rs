mod input;

use input::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1");

    //let input = _get_sample_input();
    let input = get_input();
    let machines = parse_input(input);

    let mut total_tokens = 0;

    'machines: for machine in machines {
        let ClawMachine{mut prize, a, b} = machine;
        let mut tokens = 0;
        let mut a_presses = 0;

        // press A button until prize is aligned with b button
        while prize.x % b.x != 0 || prize.y % b.y != 0 || prize.x / b.x != prize.y / b.y {
            // A button = 3 tokens
            tokens += 3;

            prize.x -= a.x;
            prize.y -= a.y;

            a_presses += 1;

            if a_presses > 100 || prize.x < 0 || prize.y < 0 {
                continue 'machines;
            }
        }

        // press B button until we get the prize
        {
            let div_x = prize.x / b.x;
            let div_y = prize.y / b.y;
            assert!(div_x == div_y);

            if div_x > 100 {
                continue 'machines;
            }

            // B button = 1 token
            tokens += div_x;
        }

        //println!("{tokens} tokens");
        total_tokens += tokens;
    }

    println!("{total_tokens}");
}

fn part2() {
    println!("Part 2");

    //let input = _get_sample_input();
    let input = get_input();
    let machines = parse_input(input);

    let mut total_tokens = 0;

    for machine in machines {
        let ClawMachine{mut prize, a, b} = machine;
        prize.x += 10000000000000;
        prize.y += 10000000000000;

        let mut tokens: i64 = 0;

        // solve for c
        let c = solve_c(&b, &prize);
        //println!("c={c}");

        // get intercept
        let (ix, iy) = get_intercept(a.y as f64 / a.x as f64, 1.0, 0.0, b.y as f64 / b.x as f64, 1.0, c);
        //println!("({ix}, {iy})");

        // round back to whole numbers, then test
        let ix = ix.round() as i64;
        let iy = iy.round() as i64;

        // line a from 0,0 to intercept
        if ix % a.x != 0 || iy % a.y != 0 {
            continue;
        }
        let div_x = ix / a.x;
        let div_y = iy / a.y;
        assert!(div_x == div_y);
        // A button = 3 tokens
        tokens += 3*div_x;

        // line b from intercept to prize
        prize.x -= ix;
        prize.y -= iy;
        if prize.x % b.x != 0 || prize.y % b.y != 0 {
            continue;
        }
        let div_x = prize.x / b.x;
        let div_y = prize.y / b.y;
        assert!(div_x == div_y);
        // B button = 1 token
        tokens += div_x;

        total_tokens += tokens;
    }

    println!("{total_tokens}");
}

fn solve_c(slope: &Position, point: &Position) -> f64 {
    let sx = slope.x as f64;
    let sy = slope.y as f64;
    let px = point.x as f64;
    let py = point.y as f64;

    py - sy*px/sx
}

fn get_intercept(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> (f64, f64) {
    let ix = (b1*c2 - b2*c1)
                / (a1*b2 - a2*b1);

    let iy = -(c1*a2 - c2*a1)
                / (a1*b2 - a2*b1);

    (ix, iy)
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut machines = vec![];

    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let line_a = lines.next().unwrap();
        let line_b = lines.next().unwrap();
        let line_prize = lines.next().unwrap();

        let a = parse_button(line_a);
        let b = parse_button(line_b);
        let prize = parse_prize(line_prize);

        machines.push(ClawMachine { a, b, prize });

        // skip blank line between entries
        lines.next();
    }

    machines
}

fn parse_button(line: &str) -> Position {
    let parts: Vec<_> = line.split(": ").collect();
    assert!(parts.len() == 2);

    let parts: Vec<_> = line.split(", ").collect();
    assert!(parts.len() == 2);

    let parts_x: Vec<_> = parts[0].split("X").collect();
    assert!(parts_x.len() == 2);
    let parts_y: Vec<_> = parts[1].split("Y").collect();
    assert!(parts_y.len() == 2);

    let x = parts_x[1].parse().unwrap();
    let y = parts_y[1].parse().unwrap();

    Position { x, y }
}

fn parse_prize(line: &str) -> Position {
    let parts: Vec<_> = line.split(": ").collect();
    assert!(parts.len() == 2);

    let parts: Vec<_> = line.split(", ").collect();
    assert!(parts.len() == 2);

    let parts_x: Vec<_> = parts[0].split("X=").collect();
    assert!(parts_x.len() == 2);
    let parts_y: Vec<_> = parts[1].split("Y=").collect();
    assert!(parts_y.len() == 2);

    let x = parts_x[1].parse().unwrap();
    let y = parts_y[1].parse().unwrap();

    Position { x, y }
}

#[derive(Debug)]
struct ClawMachine {
    a: Position,
    b: Position,
    prize: Position,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}
