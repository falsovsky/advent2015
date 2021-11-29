use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<String> {
    let mut code: Vec<String> = Vec::new();
    let f = File::open("input/day05.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        code.push(line.unwrap());
    }
    code
}

fn solve_part1(string: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const BAD: [&str; 4] = ["ab", "cd", "pq", "xy"];
    let mut valid = false;
    let cstr = string.as_bytes();

    // vowels
    let mut vc = 0;
    for vowel in VOWELS {
        for c in cstr {
            if *c as char == vowel {
                vc += 1;
            }
        }
    }

    // double
    let mut double = 0;
    for idx in 0..string.len() - 1 {
        if cstr[idx] == cstr[idx + 1] {
            double += 1;
        }
    }

    // bad
    let mut bw = 0;
    for b in BAD {
        if string.contains(b) {
            bw += 1;
        }
    }

    if vc >= 3 && double > 0 && bw == 0 {
        valid = true;
    }
    valid
}

fn solve_part2(string: &str) -> bool {
    let mut valid = false;
    let cstr = string.as_bytes();

    // rule1
    let mut rule1 = false;
    for idx in 0..string.len() - 1 {
        let pair: String = string[idx..idx+2].to_string();
        let new: String;
        if idx == 0 {
            new = string[idx + 2..string.len()].to_string();
        } else {
            new = string[0..idx].to_string() + " " + &string[idx + 2..string.len()].to_string();
        }
        if new.contains(&pair) {
            rule1 = true;
            break;
        }
    }
    
    // rule2
    let mut rule2 = false;
    for idx in 0..string.len() - 2 {
        if cstr[idx] == cstr[idx + 2] {
            rule2 = true;
            break;
        }
    }

    if rule1 && rule2 {
        valid = true;
    }
    valid
}

fn solve(strings: &[String]) -> (u32, u32) {
    let mut pt1 = 0;
    let mut pt2 = 0;
    for string in strings {
        if solve_part1(&string.to_string()) {
            pt1 += 1;
        }
        if solve_part2(&string.to_string()) {
            pt2 += 1;
        }
    }
    (pt1, pt2)
}

fn main() {
    let input = read_input();
    let (pt1, pt2) = solve(&input);
    println!("Part1: {:?}", pt1);
    println!("Part2: {:?}", pt2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let inputs = [
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false)
        ];
        for input in inputs {
            let string = String::from(input.0);
            let result = solve_part1(string);
            assert_eq!(result, input.1);
        }
    }

    #[test]
    fn part2() {
        let inputs = [
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false)
        ];
        for input in inputs {
            let string = String::from(input.0);
            let result = solve_part2(string);
            assert_eq!(result, input.1);
        }
    }
}
