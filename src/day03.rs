#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn read_input() -> Vec<char> {
    let mut code: Vec<char> = Vec::new();
    let filename = "input/day03.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    for line in buffer.lines() {
        let line_str = match line {
            Ok(value) => value,
            Err(error) => panic!("Could not read anything - {}", error),
        };
        code = line_str.to_string().chars().collect();
    }
    code
}

fn get_value(positions: &HashMap<(i32, i32), i32>, position: (i32, i32)) -> i32 {
    let value: i32;
    if positions.contains_key(&position) {
        value = match positions.get(&position) {
            Some(v) => *v,
            None => panic!("Invalid position: {},{}", position.0, position.1)
        };
    } else {
        value = 1;
    };
    value
}

fn move_it(mut actor: &mut (i32, i32), operation: char) {
    match operation {
        '>' => {
            actor.0 += 1;
        },
        '<' => {
            actor.0 -= 1;
        },
        '^' => {
            actor.1 += 1;
        },
        'v' => {
            actor.1 -= 1;
        },
        _ => panic!("Invalid operation: {:?}", operation)
    };
}

fn solve(program: &[char]) -> (i32, i32) {
    // pt1
    let mut santa1: (i32, i32) = (0, 0);
    let mut santa2: (i32, i32) = (0, 0);
    let mut robo: (i32, i32) = (0, 0);

    let mut positions_pt1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut positions_pt2: HashMap<(i32, i32), i32> = HashMap::new();

    positions_pt1.insert(santa1, 1);
    positions_pt2.insert(santa2, 2);

    let mut pc1 = 0;
    let mut pc2 = 0;
    let program_size = program.len();
    loop {
        let mut value: i32;
        // pt1
        move_it(&mut santa1, program[pc1]);
        value = get_value(&positions_pt1, santa1);
        positions_pt1.insert(santa1, value);
        pc1 += 1;

        // pt2
        if pc2 < program_size {
            move_it(&mut santa2, program[pc2]);
            pc2 += 1;
            if pc2 < program.len() {
                move_it(&mut robo, program[pc2]);
            }

            value = get_value(&positions_pt2, santa2);
            positions_pt2.insert(santa2, value);
            if pc2 < program.len() {
                value = get_value(&positions_pt2, robo);
                positions_pt2.insert(robo, value);
            }
            pc2 += 1;
        }

        if pc1 >= program_size {
            break;
        }
    }
    let pos1_len: i32 = match positions_pt1.len().try_into() {
        Ok(value) => value,
        Err(error) => panic!("Could not convert - {}", error),
    };
    let pos2_len: i32 = match positions_pt2.len().try_into() {
        Ok(value) => value,
        Err(error) => panic!("Could not convert - {}", error),
    };
    (pos1_len, pos2_len)
}

fn main() {
    let code = read_input();
    let result = solve(&code);
    println!("Part1: {:?}", result.0);
    println!("part2: {:?}", result.1);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let inputs = [
            (">", 2),
            ("^>v<", 4),
            ("^v^v^v^v^v", 2),
        ];
        for input in inputs {
            let code: Vec<char> = String::from(input.0).chars().collect();
            let (pt1, _) = solve(&code);
            assert_eq!(pt1, input.1);
        }
    }

    #[test]
    fn part2() {
        let inputs = [
            ("^v", 3),
            ("^>v<", 3),
            ("^v^v^v^v^v", 11),
        ];
        for input in inputs {
            let code: Vec<char> = String::from(input.0).chars().collect();
            let (_, pt2) = solve(&code);
            assert_eq!(pt2, input.1);
        }
    }
}

#[bench]
fn bench_day03(b: &mut test::Bencher) {
    b.iter(|| main());
}
