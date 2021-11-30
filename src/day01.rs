use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<char> {
    let filename = "input/day01.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let mut buffer = BufReader::new(&fp);
    let mut line = String::new();
    let _ = match buffer.read_line(&mut line) {
        Ok(v) => v,
        Err(error) => panic!("{}", error),
    };
    line.trim().chars().collect::<Vec<_>>()
}

fn solve(program: &[char]) -> (i16, i16) {
    let mut level: i16 = 0;
    let mut position: i16 = 0;
    let mut idx: i16 = 0;
    program.iter().enumerate().for_each(|(_, chr)| {
        match chr {
            '(' => level += 1,
            ')' => level -= 1,
            _ => panic!("Invalid instruction: {}", chr)
        }
        if position == 0 && level == -1 {
            position = idx + 1;
        }
        idx += 1;
    });
    (level, position)
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
