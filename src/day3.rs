use std::fs;

use regex::Regex;

use crate::problem::Problem;

fn get_pairs(s: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(s)
        .map(|caps| {
            let (_, [l, r]) = caps.extract();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

fn get_pairs_stateful(s: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(don't)|(do)").unwrap();
    let (_, acc) = re
        .captures_iter(s)
        .fold((true, Vec::new()), |(enabled, mut acc), caps| {
            if caps.get(4).is_some() {
                (false, acc)
            } else if caps.get(5).is_some() {
                (true, acc)
            } else if !enabled {
                (enabled, acc)
            } else {
                let l = &caps[2];
                let r = &caps[3];
                acc.push((l.parse().unwrap(), r.parse().unwrap()));
                (enabled, acc)
            }
        });

    acc
}

#[derive(Default)]
pub struct Day3 {
    data: String,
}

impl Day3 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day3.txt").unwrap();
        Day3 {
            data: data.to_owned(),
        }
    }
    fn prob1_inner(&mut self) -> usize {
        let pairs = get_pairs(&self.data);
        pairs.iter().map(|(l, r)| l * r).sum()
    }
    fn prob2_inner(&mut self) -> usize {
        let pairs = get_pairs_stateful(&self.data);
        pairs.iter().map(|(l, r)| l * r).sum()
    }
}

impl Problem for Day3 {
    fn prob1(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob1_inner())
    }

    fn prob2(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob2_inner())
    }
}

#[cfg(test)]
mod tests {

    use super::{get_pairs, get_pairs_stateful, Day3};

    const TEST_DATA: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_basics_parse() {
        let pairs = get_pairs(TEST_DATA);
        assert_eq!(pairs, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
        let res: usize = pairs.iter().map(|(l, r)| l * r).sum();
        assert_eq!(res, 161);
    }

    const TEST_DATA_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_prob2() {
        let pairs = get_pairs_stateful(TEST_DATA_2);
        assert_eq!(pairs, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn test_prob() {
        let mut day3 = Day3::new();
        assert_eq!(day3.prob1_inner(), 183380722);
        assert_eq!(day3.prob2_inner(), 82733683);
    }
}
