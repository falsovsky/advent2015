use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<(u32, u32, u32)> {
    let mut code: Vec<(u32, u32, u32)> = Vec::new();
    let filename = "input/day02.txt";
    let fp = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{} - {}", filename, error),
    };
    let buffer = BufReader::new(&fp);
    for line in buffer.lines() {
        let rline = match line {
            Ok(line) => line,
            Err(error) => panic!("{}", error),
        };
        let data = rline.split('x').collect::<Vec<&str>>();
        code.push((
            match data[0].to_string().parse::<u32>() {
                Ok(value) => value,
                Err(error) => panic!("Could not convert {} - {}", data[0], error),
            },
            match data[1].to_string().parse::<u32>() {
                Ok(value) => value,
                Err(error) => panic!("Could not convert {} - {}", data[1], error),
            },
            match data[2].to_string().parse::<u32>() {
                Ok(value) => value,
                Err(error) => panic!("Could not convert {} - {}", data[1], error),
            }
        ));
    }
    code
}

fn cal_paper(length: u32, width: u32, height: u32) -> (u32, u32) {
    let side1 = length * width;
    let side2 = width * height;
    let side3 = height * length;
    let values: Vec<u32> = [side1, side2, side3].to_vec();
    let min = match values.iter().min() {
        Some(value) => value,
        None => panic!("No minimum value"),
    };
    (2 * side1 + 2 * side2 + 2 * side3, *min)
}

fn cal_ribbon(length: u32, width: u32, height: u32) -> (u32, u32) {
    let mut values: Vec<u32> = vec![length, width , height];
    values.sort_unstable();
    let present = values[0] + values[0] + values[1] + values[1];
    let bow = values[0] * values[1] * values[2];
    (present, bow)
}

fn solve(program: &[(u32, u32, u32)]) -> (u32, u32) {
    let mut paper: u32 = 0;
    let mut slack: u32 = 0;
    let mut present: u32 = 0;
    let mut bow: u32= 0;
    for i in program {
        let (p, s) = cal_paper(i.0, i.1, i.2);
        paper += p;
        slack += s;
        let (p, b) = cal_ribbon(i.0, i.1, i.2);
        present += p;
        bow += b;
    }
    (paper + slack, present +  bow)
}

fn main() {
    let code = read_input();
    let (pt1, pt2) = solve(&code);
    println!("Part1 {}", pt1);
    println!("Part2 {}", pt2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let inputs= [
            ((2,3,4), 58),
            ((1,1,10), 43)
        ];
        for input in inputs {
            let code:Vec<(u32, u32, u32)> = vec![(input.0.0, input.0.1, input.0.2)];
            let (pt1, _) = solve(&code);
            assert_eq!(pt1, input.1);
        }
    }

    #[test]
    fn part2() {
        let inputs= [
            ((2,3,4), 34),
            ((1,1,10), 14)
        ];
        for input in inputs {
            let code:Vec<(u32, u32, u32)> = vec![(input.0.0, input.0.1, input.0.2)];
            let (_, pt2) = solve(&code);
            assert_eq!(pt2, input.1);
        }
    }
}
