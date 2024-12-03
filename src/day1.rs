use std::collections::HashMap;

use sorted_vec::SortedVec;

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

fn prob1(data: &str) -> i64 {
    let (l, r) = deserialize(data);
    count_distance(&l, &r)
}

fn prob2(data: &str) -> usize {
    let (l, r) = deserialize(data);
    let f = frequency(&r);
    l.iter().map(|n| *n as usize * f.get(n).unwrap_or(&0)).sum()
}

fn frequency(d: &SortedVec<i64>) -> HashMap<i64, usize> {
    d.chunk_by(|a, b| a == b).map(|s| (s[0], s.len())).collect()
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::day1::{count_distance, deserialize, frequency, prob1, prob2};

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
    fn test_problem_1() {
        let data = fs::read_to_string("data/day1.txt").unwrap();

        let res = prob1(&data);
        assert_eq!(res, 2000468);
    }

    #[test]
    fn test_frequency() {
        let (_, right) = deserialize(TEST_DATA);

        let f = frequency(&right);

        assert_eq!(f, HashMap::from([(3, 3), (4, 1), (5, 1), (9, 1)]));
    }

    #[test]
    fn test_problem_2_test() {
        let res = prob2(TEST_DATA);
        assert_eq!(res, 31);
    }

    #[test]
    fn test_problem_2() {
        let data = fs::read_to_string("data/day1.txt").unwrap();

        let res = prob2(&data);
        assert_eq!(res, 18567089);
    }
}
