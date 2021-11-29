fn solve(key: &str, second: bool) -> (u32, u32) {
    let mut idx = 0;
    let mut pt1 = 0;
    let mut pt2 = 0;
    loop {
        let fullkey = format!("{}{}", key, idx);
        let digest = md5::compute(fullkey);
        let hex = format!("{:x}", digest);
        if pt1 == 0 && hex[0..5] == "0".repeat(5) {
            pt1 = idx;
            if !second {
                break;
            }
        }
        if second && pt2 == 0 && hex[0..6] == "0".repeat(6) {
            pt2 = idx;
            break;
        }
        idx += 1;
    }
    (pt1, pt2)
}

fn main() {
    let input = String::from("ckczppom");
    let result = solve(&input, true);
    println!("Part1: {:?}", result.0);
    println!("Part2: {:?}", result.1);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let inputs = [
            ("abcdef", 609043, false),
            ("pqrstuv", 1048970, false),
        ];
        for input in inputs {
            let key = String::from(input.0);
            let result = solve(&key, input.2);
            assert_eq!(result.0, input.1);
        }
    }
}
