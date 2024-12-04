use std::{collections::HashMap, convert::identity, fs};

use linkme::distributed_slice;

use crate::problem::{Problem, PROBLEMS};

fn parse(s: &str) -> Vec<Vec<usize>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|ns| ns.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(last: usize, next: usize, asc: Option<bool>) -> bool {
    let diff = last.abs_diff(next);
    if diff > 3 || diff < 1 {
        false
    } else {
        // We know they aren't the same because diff >= 1
        let increasing = next > last;
        match asc {
            None => true,
            Some(true) => increasing,
            Some(false) => !increasing,
        }
    }
}

#[derive(Debug, Clone)]
enum Report {
    Unstarted,
    Initial { last: usize },
    Safe { last: usize, asc: bool },
    Failed,
}

impl Report {
    fn include(&self, next: usize) -> Report {
        match self {
            Report::Failed { .. } => self.clone(),
            Report::Unstarted => Report::Initial { last: next },
            Report::Initial { last } => {
                if is_safe(*last, next, None) {
                    let asc = next > *last;
                    Report::Safe { last: next, asc }
                } else {
                    Report::Failed
                }
            }
            Report::Safe { last, asc } => {
                if is_safe(*last, next, Some(*asc)) {
                    Report::Safe {
                        last: next,
                        asc: *asc,
                    }
                } else {
                    Report::Failed
                }
            }
        }
    }
}

impl From<Report> for bool {
    fn from(item: Report) -> Self {
        match item {
            Report::Unstarted => false,
            Report::Initial { .. } => true,
            Report::Safe { .. } => true,
            Report::Failed => false,
        }
    }
}

fn is_safe_report(data: &[usize]) -> Report {
    data.iter()
        .fold(Report::Unstarted, |report, n| report.include(*n))
}

struct Day2 {
    data: Vec<Vec<usize>>,
}

impl Day2 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day2.txt").unwrap();
        Day2::with_data(&data)
    }
    fn with_data(s: &str) -> Self {
        Day2 { data: parse(s) }
    }
    fn prob1_inner(&mut self) -> usize {
        self.data
            .iter()
            .map(|v| if is_safe_report(v).into() { 1 } else { 0 })
            .sum()
    }
    fn prob2_inner(&mut self) -> usize {
        self.data
            .iter()
            .map(|v| match is_safe_report(&v) {
                Report::Failed => {
                    if (0..v.len())
                        .map(|n| {
                            let mut c = v.clone();
                            c.remove(n);
                            is_safe_report(&c).into()
                        })
                        .any(identity)
                    {
                        1
                    } else {
                        0
                    }
                }
                _ => 1,
            })
            .sum()
    }
}

impl Problem for Day2 {
    fn prob1(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob1_inner())
    }

    fn prob2(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob2_inner())
    }
}

#[distributed_slice(PROBLEMS)]
fn register_day(p: &mut HashMap<String, fn() -> Box<dyn Problem>>) {
    p.insert("day2".to_owned(), || Box::new(Day2::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

    #[test]
    fn test_basic_parse() {
        let res = parse("1 2\n3 4");

        assert_eq!(res, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_basic_safe() {
        assert_eq!(is_safe(1, 1, None), false);
        assert_eq!(is_safe(1, 1, Some(true)), false);
        assert_eq!(is_safe(1, 1, Some(false)), false);
        assert_eq!(is_safe(1, 5, None), false);
        assert_eq!(is_safe(1, 5, Some(true)), false);
        assert_eq!(is_safe(1, 5, Some(false)), false);
        assert_eq!(is_safe(9, 5, None), false);
        assert_eq!(is_safe(9, 5, Some(true)), false);
        assert_eq!(is_safe(9, 5, Some(false)), false);
        assert_eq!(is_safe(0, 1, None), true);
        assert_eq!(is_safe(1, 0, None), true);
        assert_eq!(is_safe(0, 1, Some(true)), true);
        assert_eq!(is_safe(0, 1, Some(false)), false);
        assert_eq!(is_safe(1, 0, Some(true)), false);
        assert_eq!(is_safe(1, 0, Some(false)), true);
    }

    #[test]
    fn test_basic_report() {
        assert!(matches!(is_safe_report(&vec![1, 2]), Report::Safe { .. }));
        assert!(matches!(is_safe_report(&vec![1, 5]), Report::Failed));
    }

    #[test]
    fn test_example() {
        let data = parse(TEST_DATA);
        assert_eq!(
            data.into_iter()
                .map(|v| is_safe_report(&v).into())
                .collect::<Vec<bool>>(),
            vec![true, false, false, false, false, true]
        );
    }

    #[test]
    fn test_example_prob1() {
        let mut day2 = Day2::with_data(TEST_DATA);
        assert_eq!(day2.prob1_inner(), 2);
    }

    #[test]
    fn test_prob2() {
        let mut day2 = Day2::with_data(TEST_DATA);
        assert_eq!(day2.prob2_inner(), 4);
    }

    #[test]
    fn test_day2() {
        let mut day2 = Day2::new();
        assert_eq!(day2.prob1_inner(), 282);
        assert_eq!(day2.prob2_inner(), 349);
    }
}
