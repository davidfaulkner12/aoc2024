use regex::Regex;

// I'll admit I totally stole this from here:
// https://github.com/jixunmoe/aoc-2024/blob/main/aoc-2024/day-13/README.MD#part-1
// I knew that you could come up with an equation to solve the matrix, I just hadn't
// worked out all the details.
fn solve(x1: i64, x2: i64, xp: i64, y1: i64, y2: i64, yp: i64) -> Option<(i64, i64)> {
    let b = (xp * y1 - yp * x1) / (x2 * y1 - y2 * x1);
    let a = (xp - b * x2) / x1;

    // Check integer math
    if (a * x1 + b * x2 == xp) && (a * y1 + b * y2 == yp) {
        Some((a, b))
    } else {
        None
    }
}

fn parse(data: &str) -> Vec<[i64; 6]> {
    let re = Regex::new(r"(?ms).*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+)").unwrap();
    let mut res = Vec::new();
    for section in data.split("\n\n") {
        let caps = re.captures(section).unwrap();
        res.push([
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
            caps[5].parse().unwrap(),
            caps[6].parse().unwrap(),
        ]);
    }
    res
}

fn cost(ns: [i64; 6]) -> Option<i64> {
    let sol = solve(ns[0], ns[2], ns[4], ns[1], ns[3], ns[5]);
    sol.map(|(a, b)| 3 * a + b)
}

#[cfg(test)]
mod tests {

    use std::{convert::identity, fs};

    use super::*;

    const TEST_DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_day13_solve_examples() {
        assert_eq!(solve(94, 22, 8400, 34, 67, 5400), Some((80, 40)));
        assert_eq!(solve(26, 67, 12748, 66, 21, 12176), None);
        assert_eq!(solve(17, 84, 7870, 86, 37, 6450), Some((38, 86)));
        assert_eq!(solve(69, 27, 18641, 23, 71, 10279), None);
    }

    #[test]
    fn test_day13_parse() {
        assert_eq!(
            parse(TEST_DATA),
            vec![
                [94, 34, 22, 67, 8400, 5400],
                [26, 66, 67, 21, 12748, 12176],
                [17, 86, 84, 37, 7870, 6450],
                [69, 23, 27, 71, 18641, 10279],
            ]
        );
    }

    #[test]
    fn test_day13_cost_example() {
        let data = parse(TEST_DATA);

        let res: Vec<_> = data.into_iter().map(cost).collect();

        assert_eq!(res, vec![Some(280), None, Some(200), None]);
    }

    #[test]
    fn test_day13_actual() {
        let data = fs::read_to_string("data/day13.txt").unwrap();
        let data = parse(&data);

        let res: i64 = data.iter().map(|ns| cost(*ns)).filter_map(identity).sum();

        assert_eq!(res, 27157);

        let res: i64 = data
            .iter()
            .map(|ns| {
                cost([
                    ns[0],
                    ns[1],
                    ns[2],
                    ns[3],
                    ns[4] + 10000000000000,
                    ns[5] + 10000000000000,
                ])
            })
            .filter_map(identity)
            .sum();

        assert_eq!(res, 104015411578548);
    }
}
