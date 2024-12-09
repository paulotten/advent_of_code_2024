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
    
    let mut filesystem = build_filesystem(parsed);
    compress(&mut filesystem);

    let checksum = get_checksum(&filesystem);

    println!("{checksum}");
}

fn part2() {
    println!("Part 2");

    let input = get_input();
    let parsed = parse_input(input);
    
    let mut filesystem = build_filesystem(parsed);
    compress2(&mut filesystem);

    //println!("{filesystem:?}");

    let checksum = get_checksum(&filesystem);

    println!("{checksum}");
}

fn parse_input(input: &str) -> Vec<u8> {
    input.chars().map(|i| i.to_digit(10).unwrap() as u8).collect()
}

fn build_filesystem(parsed: Vec<u8>) -> Vec<Option<u32>> {
    let mut filesystem = vec![];

    let mut is_free_space = false;
    let mut id: u32 = 0;

    for val in parsed {
        if is_free_space {
            for _ in 0..val {
                filesystem.push(None);
            }
        } else {
            for _ in 0..val {
                filesystem.push(Some(id));
            }

            id += 1;
        }

        is_free_space = !is_free_space;
    }

    filesystem
}

fn compress(filesystem: &mut Vec<Option<u32>>) {
    let mut i = 0;
    let mut j = filesystem.len() - 1;

    loop {
        while filesystem[i].is_some() {
            i += 1;
        }

        while filesystem[j].is_none() {
            j -= 1;
        }

        if i >= j {
            break;
        }

        filesystem[i] = filesystem[j];
        filesystem[j] = None;
    }
}

fn compress2(filesystem: &mut Vec<Option<u32>>) {
    let mut j = filesystem.len() - 1;

    loop {
        while filesystem[j].is_none() {
            j -= 1;
        }

        let val = filesystem[j];

        let mut i = j;
        while i > 0 && filesystem[i-1] == val {
            i -= 1;
        }

        let size = j-i+1;
        let gap = get_gap(size, filesystem, i);

        //println!("{val:?} {i} to {j}");
        //println!("size={size} gap={gap:?}");

        if let Some(gap_start) = gap {
            for i2 in 0..size {
                filesystem[gap_start+i2] = filesystem[i+i2];
                filesystem[i+i2] = None;
            }
        }

        if i == 0 {
            break;
        }

        j = i-1;
    }
}

/*
Get first big enough gap before current file
*/
fn get_gap(size: usize, filesystem: &Vec<Option<u32>>, end: usize) -> Option<usize> {
    if end >= size {
        'outer: for i in 0..=end-size {
            for j in 0..size {
                if filesystem[i+j].is_some() {
                    continue 'outer;
                }
            }

            return Some(i);
        }
    }

    None
}

fn get_checksum(filesystem: &Vec<Option<u32>>) -> u64 {
    let mut checksum = 0;

    for i in 0..filesystem.len() {
        if let Some(val) = filesystem[i] {
            checksum += i as u64 * val as u64;
        }
    }

    checksum
}
