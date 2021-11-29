use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<(u32, u32, u32)> {
    let mut code: Vec<(u32, u32, u32)> = Vec::new();
    let f = File::open("input/day02.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let data = l.split('x').collect::<Vec<&str>>();
        code.push((
            data[0].to_string().parse::<u32>().unwrap(),
            data[1].to_string().parse::<u32>().unwrap(),
            data[2].to_string().parse::<u32>().unwrap()
        ));
    }
    code
}

fn cal_paper(length: u32, width: u32, height: u32) -> (u32, u32) {
    let side1 = length * width;
    let side2 = width * height;
    let side3 = height * length;
    let values: Vec<u32> = [side1, side2, side3].to_vec();
    let min = values.iter().min().unwrap();
    (2 * side1 + 2 * side2 + 2 * side3, *min)
}

fn cal_ribbon(length: u32, width: u32, height: u32) -> (u32, u32) {
    let mut values: Vec<u32> = vec![length, width , height];
    values.sort_unstable();
    let present = values[0] + values[0] + values[1] + values[1];
    let bow = values[0] * values[1] * values[2];
    (present, bow)
}

fn solve(program: &[(u32, u32, u32)]) -> (u32, u32, u32, u32) {
    let mut paper = 0;
    let mut slack = 0;
    let mut present = 0;
    let mut bow = 0;
    for i in program {
        let (p, s) = cal_paper(i.0, i.1, i.2);
        paper += p;
        slack += s;
        let (p, b) = cal_ribbon(i.0, i.1, i.2);
        present += p;
        bow += b;
    }
    (paper, slack, present, bow)
}

fn main() {
    let code = read_input();
    let (paper, slack, present, bow) = solve(&code);
    println!("Part1 {:?} - Paper: {:?}, Slack: {:?}", paper + slack, paper, slack);
    println!("Part2 {:?} - Present: {:?}, Bow: {:?}", present + bow, present, bow);
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
            let (paper, slack, _, _) = solve(&code);
            assert_eq!(paper + slack, input.1);
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
            let (_, _, present, bow) = solve(&code);
            assert_eq!(present + bow, input.1);
        }
    }
}
