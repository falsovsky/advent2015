use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<char> {
    let mut code: Vec<char> = Vec::new();
    let f = File::open("input/day01.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        for char in line.unwrap().to_string().chars() {
            code.push(char);
        }
    }
    code
}

fn solve(program: &[char]) -> (i32, i32) {
    let mut level = 0;
    let mut position = 0;
    for (i, c) in program.iter().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => panic!("Invalid instruction: {:?}", c)
        }
        if position == 0 && level == -1 {
            position = i + 1;
        }
    }
    (level, position.try_into().unwrap())
}

fn main() {
    let code = read_input();
    let (level, position) = solve(&code);
    println!("Part1: {:?}", level);
    println!("Part2: {:?}", position);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let inputs = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3)
        ];
        for input in inputs {
            let code: Vec<char> = String::from(input.0).chars().collect();
            let (level, _) = solve(&code);
            assert_eq!(level, input.1);
        }
    }

    #[test]
    fn part2() {
        let inputs = [
            (")", 1),
            ("()())", 5),
        ];
        for input in inputs {
            let code: Vec<char> = String::from(input.0).chars().collect();
            let (_, position) = solve(&code);
            assert_eq!(position, input.1);
        }
    }
}
