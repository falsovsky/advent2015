use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;

type Type = (String, (u32, u32), (u32, u32));
type Program = Vec<Type>;

fn read_input() -> Program {
    let mut code: Program = Vec::new();
    let re = Regex::new(r"(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let f = File::open("input/day06.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            code.push(
                (
                    cap[1].to_string(),
                    (cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap()),
                    (cap[4].parse::<u32>().unwrap(), cap[5].parse::<u32>().unwrap())
                )
            );
        }
    }
    code
}

fn solve1(code: &[Type]) -> u32 {
    let mut lights: HashMap<(u32, u32), bool> = HashMap::new();
    for instruction in code {
        let start = instruction.1;
        let finish = instruction.2;
        for x in start.0..=finish.0 as u32{
            for y in start.1..=finish.1 as u32{
                let mut val = lights.get(&(x, y)).copied().unwrap_or(false);
                match instruction.0.as_str() {
                    "on" => val = true,
                    "off" => val = false,
                    "toggle" => val = !val,
                    _ => panic!("Invalid instruction - {}", instruction.0)
                }
                lights.insert((x, y), val);
            }
        }
    }
    let mut total = 0;
    for light in lights {
        if light.1 {
            total += 1;
        }
    }
    total
}

fn solve2(code: &[Type]) -> u32 {
    let mut lights: HashMap<(u32, u32), u32> = HashMap::new();
    for instruction in code {
        let start = instruction.1;
        let finish = instruction.2;
        for x in start.0..=finish.0 as u32 {
            for y in start.1..=finish.1 as u32 {
                let mut val = lights.get(&(x, y)).copied().unwrap_or(0);
                match instruction.0.as_str() {
                    "on" => val += 1,
                    "off" => val = val.saturating_sub(1),
                    "toggle" => val += 2,
                    _ => panic!("Invalid instruction - {}", instruction.0)
                }
                lights.insert((x, y), val);
            }
        }
    }
    let mut total = 0;
    for light in lights {
        total += light.1;
    }
    total
}

fn main() {
    let input = read_input();
    let pt1 = solve1(&input);
    println!("Part1: {:?}", pt1);
    let pt2 = solve2(&input);
    println!("Part2: {:?}", pt2);
}
