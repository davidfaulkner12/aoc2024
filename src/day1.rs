use std::{collections::HashMap, fs};

use linkme::distributed_slice;
use sorted_vec::SortedVec;

use crate::{problem::Problem, PROBLEMS};

fn deserialize(s: &str) -> (SortedVec<i64>, SortedVec<i64>) {
    let mut left = SortedVec::new();
    let mut right = SortedVec::new();
    for line in s.lines() {
        let mut ns = line.split_whitespace();

        left.push(ns.next().unwrap().parse().unwrap());
        right.push(ns.next().unwrap().parse().unwrap());
    }
    return (left, right);
}

fn count_distance(left: &[i64], right: &[i64]) -> i64 {
    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn multiply_frequency(left: &[i64], f: HashMap<i64, usize>) -> usize {
    left.iter()
        .map(|n| *n as usize * f.get(n).unwrap_or(&0))
        .sum()
}

fn frequency(d: &SortedVec<i64>) -> HashMap<i64, usize> {
    d.chunk_by(|a, b| a == b).map(|s| (s[0], s.len())).collect()
}

#[derive(Default)]
pub struct Day1 {
    left: SortedVec<i64>,
    right: SortedVec<i64>,
}

impl Day1 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day1.txt").unwrap();
        let (left, right) = deserialize(&data);
        Day1 { left, right }
    }
    fn prob1_inner(&mut self) -> i64 {
        count_distance(&self.left, &self.right)
    }
    fn prob2_inner(&mut self) -> usize {
        let f = frequency(&self.right);
        multiply_frequency(&self.left, f)
    }
}

impl Problem for Day1 {
    fn prob1(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob1_inner())
    }

    fn prob2(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob2_inner())
    }
}

#[distributed_slice(PROBLEMS)]
fn register_day(p: &mut HashMap<String, fn() -> Box<dyn Problem>>) {
    p.insert("day1".to_owned(), || Box::new(Day1::new()));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day1::{count_distance, deserialize, frequency, Day1};

    const TEST_DATA: &str = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";

    #[test]
    fn test_basic_decode() {
        let (left, right) = deserialize(TEST_DATA);

        assert_eq!(left.as_slice(), vec![1, 2, 3, 3, 3, 4]);
        assert_eq!(right.as_slice(), vec![3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn test_basic_distance() {
        let (left, right) = deserialize(TEST_DATA);

        let res = count_distance(&left, &right);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_frequency() {
        let (_, right) = deserialize(TEST_DATA);

        let f = frequency(&right);

        assert_eq!(f, HashMap::from([(3, 3), (4, 1), (5, 1), (9, 1)]));
    }

    #[test]
    fn test_problem_2_test() {
        let (left, right) = deserialize(TEST_DATA);
        let mut day1 = Day1 { left, right };
        let res = day1.prob2_inner();
        assert_eq!(res, 31);
    }

    #[test]
    fn test_problem_2() {
        let mut day1 = Day1::new();

        let res = day1.prob1_inner();
        assert_eq!(res, 2000468);

        let res = day1.prob2_inner();
        assert_eq!(res, 18567089);
    }
}
