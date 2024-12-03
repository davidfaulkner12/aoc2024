use std::convert::identity;

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

fn prob1(data: &str) -> usize {
    let data = parse(data);
    data.into_iter()
        .map(|v| if is_safe_report(&v).into() { 1 } else { 0 })
        .sum()
}

fn prob2(data: &str) -> usize {
    let data = parse(data);
    data.into_iter()
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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day2::{is_safe, is_safe_report, parse, prob1, prob2, Report};

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
    fn test_example_day1() {
        assert_eq!(prob1(TEST_DATA), 2);
    }

    #[test]
    fn test_day1() {
        let data = fs::read_to_string("data/day2.txt").unwrap();
        assert_eq!(prob1(&data), 282);
    }
    #[test]
    fn test_example_prob2() {
        assert_eq!(prob2(TEST_DATA), 4);
    }

    #[test]
    fn test_prob2() {
        let data = fs::read_to_string("data/day2.txt").unwrap();
        assert_eq!(prob2(&data), 349);
    }
}
