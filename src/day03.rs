use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn read_input() -> Vec<char> {
    let mut code: Vec<char> = Vec::new();
    let f = File::open("input/day03.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        for char in line.unwrap().to_string().chars() {
            code.push(char);
        }
    }
    code
}

fn get_value(positions: &HashMap<(i32, i32), i32>, position: (i32, i32)) -> i32 {
    let value: i32;
    if positions.contains_key(&position) {
        value = *positions.get(&position).unwrap();
    } else {
        value = 1;
    };
    value
}

fn solve(program: &[char]) -> (i32, i32) {
    // pt1
    let mut santa: (i32, i32) = (0, 0);
    let mut positions_pt1: HashMap<(i32, i32), i32> = HashMap::new();
    positions_pt1.insert(santa, 1);
    for char in program {
        match char {
            '>' => {
                santa.0 += 1;
            },
            '<' => {
                santa.0 -= 1;
            },
            '^' => {
                santa.1 += 1;
            },
            'v' => {
                santa.1 -= 1;
            },
            _ => panic!("Invalid operation: {:?}", char)
        }
        let value = get_value(&positions_pt1, santa);
        positions_pt1.insert(santa, value);
    }

    // pt2
    santa = (0, 0);
    let mut robo: (i32, i32) = (0, 0);
    let mut positions_pt2: HashMap<(i32, i32), i32> = HashMap::new();
    positions_pt2.insert(santa, 2);
    let mut pc = 0;
    while pc < program.len() {
        match program[pc] {
            '>' => {
                santa.0 += 1;
            },
            '<' => {
                santa.0 -= 1;
            },
            '^' => {
                santa.1 += 1;
            },
            'v' => {
                santa.1 -= 1;
            },
            _ => panic!("Invalid operation: {:?}", program[pc])
        }
        pc += 1;
        if pc < program.len() {
            match program[pc] {
                '>' => {
                    robo.0 += 1;
                },
                '<' => {
                    robo.0 -= 1;
                },
                '^' => {
                    robo.1 += 1;
                },
                'v' => {
                    robo.1 -= 1;
                },
                _ => panic!("Invalid operation: {:?}", program[pc])
            }
        }

        let mut value = get_value(&positions_pt2, santa);
        positions_pt2.insert(santa, value);
        if pc < program.len() {
            value = get_value(&positions_pt2, robo);
            positions_pt2.insert(robo, value);
        }
        pc += 1;
    }
    (positions_pt1.len().try_into().unwrap(), positions_pt2.len().try_into().unwrap())
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
