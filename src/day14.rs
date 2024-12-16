use itertools::*;
use regex::Regex;

// Technically this is a u64 but I'm so sick of the hoops with unsigned math
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(i64, i64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Velocity(i64, i64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Robot(Point, Velocity);

fn wrap(value: i64, limit: i64) -> i64 {
    if value < 0 {
        limit + value
    } else {
        value % limit
    }
}

fn score(rs: Vec<Robot>, corner: Point) -> i64 {
    // This only works if our corner is even, but in this problem it always is
    let center = Point(corner.0 / 2, corner.1 / 2);

    let mut q_counts = [0, 0, 0, 0];

    for r in rs {
        if r.0 .0 < center.0 && r.0 .1 < center.1 {
            q_counts[0] += 1;
        } else if r.0 .0 < center.0 && r.0 .1 > center.1 {
            q_counts[1] += 1;
        } else if r.0 .0 > center.0 && r.0 .1 < center.1 {
            q_counts[2] += 1;
        } else if r.0 .0 > center.0 && r.0 .1 > center.1 {
            q_counts[3] += 1;
        }
    }

    q_counts.iter().product()
}

impl Robot {
    fn tick(&self, corner: Point) -> Robot {
        let next = Point(
            wrap(self.0 .0 + self.1 .0, corner.0 + 1),
            wrap(self.0 .1 + self.1 .1, corner.1 + 1),
        );
        Robot(next, self.1)
    }
}

fn parse(data: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    data.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Robot(
                Point(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                Velocity(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            )
        })
        .collect()
}

fn print_robots(rs: &Vec<Robot>, corner: Point) {
    let cs = rs.into_iter().map(|r| r.0).counts();

    for x in 0..=corner.0 {
        for y in 0..=corner.1 {
            if let Some(c) = cs.get(&Point(x, y)) {
                print!("{}", c);
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{fs, thread};

    use super::*;

    const TEST_DATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_parse() {
        let test = "p=0,4 v=3,-3";
        assert_eq!(parse(test), vec![Robot(Point(0, 4), Velocity(3, -3))]);
    }

    #[test]
    fn test_wrap() {
        let corner = Point(10, 6);
        let v = Velocity(2, -3);
        let r = Robot(Point(2, 4), v);

        let r = r.tick(corner);
        assert_eq!(r, Robot(Point(4, 1), v));

        let r = r.tick(corner);
        assert_eq!(r, Robot(Point(6, 5), v));

        let r = r.tick(corner);
        assert_eq!(r, Robot(Point(8, 2), v));

        let r = r.tick(corner);
        assert_eq!(r, Robot(Point(10, 6), v));

        let r = r.tick(corner);
        assert_eq!(r, Robot(Point(1, 3), v));
    }

    #[test]
    fn test_run_14() {
        let mut rs = parse(TEST_DATA);

        let corner = Point(10, 6);

        for _ in 0..100 {
            rs = rs.into_iter().map(|r| r.tick(corner)).collect();
        }

        assert_eq!(score(rs, corner), 12);
    }

    #[test]
    fn test_actual_14() {
        let data = fs::read_to_string("data/day14.txt").unwrap();
        let mut rs = parse(&data);

        let corner = Point(100, 102);

        for _ in 0..100 {
            rs = rs.into_iter().map(|r| r.tick(corner)).collect();
        }

        assert_eq!(score(rs, corner), 229632480);
    }

    fn heuristic(rs: &Vec<Robot>) -> bool {
        let ys = rs.iter().map(|r| r.0 .1).counts();

        for v in ys.values() {
            if *v > 25 {
                return true;
            }
        }
        return false;
    }

    #[test]
    fn test_run_14_part2() {
        let data = fs::read_to_string("data/day14.txt").unwrap();
        let mut rs = parse(&data);

        let corner = Point(100, 102);

        let one_second = time::Duration::from_millis(1000);

        for i in 0..10000 {
            rs = rs.into_iter().map(|r| r.tick(corner)).collect();

            if heuristic(&rs) {
                println!("{}", i + 1);
                print_robots(&rs, corner);
                thread::sleep(one_second);
            }
        }

        // The answer is 7051 and it's really obvious which one it is

        //assert_eq!(score(rs, corner), 12);
    }
}
