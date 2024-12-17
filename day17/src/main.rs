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
    let mut computer = parse_input(input);

    run_computer(&mut computer);

    print_output(&computer.output);
}

fn part2() {
    println!("Part 2");

    //let input = _get_sample_input2();
    let input = get_input();
    let mut computer = parse_input(input);

    let mut a = 0;

    //print_output(&computer.program);

    loop {
        computer.output = vec![];
        computer.register_a = a;

        run_computer(&mut computer);

        if computer.output == computer.program {
            println!("{a}");
            break;
        }

        if computer.program.ends_with(&computer.output) {
            //print_output(&computer.output);
            //println!("{a:#b}");

            a *= 8;
            continue;
        }

        a += 1;
    }
}

#[derive(Debug)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<u8>,
    output: Vec<u8>,
}

fn parse_input(input: &str) -> Computer {
    let mut lines = input.lines();

    let parts: Vec<_> = lines.next().unwrap().split("Register A: ").collect();
    assert!(parts.len() == 2);
    let register_a = parts[1].parse().unwrap();

    let parts: Vec<_> = lines.next().unwrap().split("Register B: ").collect();
    assert!(parts.len() == 2);
    let register_b = parts[1].parse().unwrap();

    let parts: Vec<_> = lines.next().unwrap().split("Register C: ").collect();
    assert!(parts.len() == 2);
    let register_c = parts[1].parse().unwrap();

    lines.next().unwrap();

    let parts: Vec<_> = lines.next().unwrap().split("Program: ").collect();
    assert!(parts.len() == 2);
    let program = parts[1].split(',').map(|o| o.parse().unwrap()).collect();

    Computer {
        register_a,
        register_b,
        register_c,
        program,
        output: vec![],
    }
}

fn combo_operand(operand: u8, computer: &Computer) -> i64 {
    match operand {
        0 | 1 | 2 | 3 => operand as i64,
        4 => computer.register_a,
        5 => computer.register_b,
        6 => computer.register_c,
        _ => panic!("Unsupported combo operand: {operand}")
    }
}

fn print_output(output: &Vec<u8>) {
    let mut first = true;

    for o in output {
        if !first {
            print!(",");
        }
        first = false;

        print!("{o}");
    }
    println!();
}

fn run_computer(computer: &mut Computer) {
    let mut instruction_pointer = 0;

    while instruction_pointer < computer.program.len() {
        let opcode = computer.program[instruction_pointer];
        let operand = computer.program[instruction_pointer+1];

        match opcode {
            0 => {
                // adv
                let numerator = computer.register_a;
                let denominator = 2_i64.pow(combo_operand(operand, &computer) as u32);

                computer.register_a = numerator / denominator;
            }
            1 => {
                // bxl
                let xor = computer.register_b ^ operand as i64;

                computer.register_b = xor;
            }
            2 => {
                // bst
                let combo = combo_operand(operand, &computer) % 8;

                computer.register_b = combo;
            }
            3 => {
                // jnz
                if computer.register_a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => {
                // bxc
                let xor = computer.register_b ^ computer.register_c;

                computer.register_b = xor;
            }
            5 => {
                // out
                let out = (combo_operand(operand, &computer) % 8) as u8;
                
                //println!("{out}");
                computer.output.push(out);
            }
            7 => {
                // cdv
                let numerator = computer.register_a;
                let denominator = 2_i64.pow(combo_operand(operand, &computer) as u32);

                computer.register_c = numerator / denominator;
            }
            _ => panic!("Unsupported opcode: {opcode}")
        };

        instruction_pointer += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            program: vec![2,6],
            output: vec![],
        };

        run_computer(&mut computer);

        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn example2() {
        let mut computer = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            program: vec![5,0,5,1,5,4],
            output: vec![],
        };

        run_computer(&mut computer);

        assert_eq!(computer.output, vec![0,1,2]);
    }

    #[test]
    fn example3() {
        let mut computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program: vec![0,1,5,4,3,0],
            output: vec![],
        };

        run_computer(&mut computer);

        assert_eq!(computer.output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(computer.register_a, 0);
    }

    #[test]
    fn example4() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            program: vec![1,7],
            output: vec![],
        };

        run_computer(&mut computer);

        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn example5() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            program: vec![4,0],
            output: vec![],
        };

        run_computer(&mut computer);

        assert_eq!(computer.register_b, 44354);
    }
}
