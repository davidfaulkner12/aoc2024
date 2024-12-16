use std::collections::HashMap;

use itertools::*;

fn maybe_split_even(i: usize) -> Option<Vec<usize>> {
    if i == 0 {
        return None;
    }

    let base = i.ilog10();

    if base % 2 == 1 {
        let split = 10usize.pow(base / 2 + 1);
        Some(vec![i / split, i % split])
    } else {
        None
    }
}

fn maybe_replace_zero(i: usize) -> Option<Vec<usize>> {
    if i == 0 {
        Some(vec![1])
    } else {
        None
    }
}

fn always_multiply_by_2024(i: usize) -> Option<Vec<usize>> {
    Some(vec![i * 2024])
}

fn apply_rules(i: usize) -> Vec<usize> {
    maybe_replace_zero(i)
        .or_else(|| maybe_split_even(i))
        .or_else(|| always_multiply_by_2024(i))
        .unwrap()
}

fn blink(ns: Vec<usize>) -> Vec<usize> {
    ns.into_iter().flat_map(apply_rules).collect()
}

fn blink25(ns: Vec<usize>) -> Vec<usize> {
    let mut ns = ns;
    for _ in 0..25 {
        ns = blink(ns);
    }
    ns
}

fn blink_with_counts(counts: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next = HashMap::new();
    for (k, v) in counts.iter() {
        apply_rules(*k).into_iter().for_each(|s| {
            next.insert(s, next.get(&s).unwrap_or(&0) + v);
        })
    }
    next
}

fn blink75(ns: Vec<usize>) -> HashMap<usize, usize> {
    let mut ns = ns.into_iter().counts();
    println!("{ns:?}");
    for _ in 0..75 {
        ns = blink_with_counts(&ns);
    }
    ns
}

#[cfg(test)]
mod tests {
    use std::{convert::identity, fs};

    use super::*;

    const TEST_DATA: &str = "0 1 10 99 999";

    #[test]
    fn test_maybe_split_even() {
        assert_eq!(maybe_split_even(0), None);
        assert_eq!(maybe_split_even(9), None);
        assert_eq!(maybe_split_even(100), None);
        assert_eq!(maybe_split_even(999), None);

        assert_eq!(maybe_split_even(10), Some(vec![1, 0]));
        assert_eq!(maybe_split_even(19), Some(vec![1, 9]));
        assert_eq!(maybe_split_even(99), Some(vec![9, 9]));
        assert_eq!(maybe_split_even(1000), Some(vec![10, 0]));
        assert_eq!(maybe_split_even(9999), Some(vec![99, 99]));
    }

    #[test]
    fn test_first_example_day11() {
        let ns: Vec<usize> = TEST_DATA
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let step1: Vec<_> = ns.into_iter().flat_map(apply_rules).collect();
        assert_eq!(step1, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_second_example_day11() {
        let ns = vec![125, 17];

        let step1 = blink(ns);
        assert_eq!(step1, vec![253000, 1, 7]);

        let step2 = blink(step1);
        assert_eq!(step2, vec![253, 0, 2024, 14168]);

        let step3 = blink(step2);
        assert_eq!(step3, vec![512072, 1, 20, 24, 28676032]);

        let step4 = blink(step3);
        assert_eq!(step4, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);

        let step5 = blink(step4);
        assert_eq!(
            step5,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );

        let mut step6 = blink(step5);
        assert_eq!(
            step6,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );

        for _ in 7..=25 {
            step6 = blink(step6);
        }

        assert_eq!(step6.len(), 55312);

        let ns = vec![125, 17];
        assert_eq!(blink25(ns).len(), 55312);
    }

    #[test]
    fn test_second_example_day11_counts() {
        let ns = vec![125, 17].into_iter().counts();

        let step1 = blink_with_counts(&ns);
        assert_eq!(step1, vec![253000, 1, 7].into_iter().counts());

        let step2 = blink_with_counts(&step1);
        assert_eq!(step2, vec![253, 0, 2024, 14168].into_iter().counts());

        let step3 = blink_with_counts(&step2);
        assert_eq!(
            step3,
            vec![512072, 1, 20, 24, 28676032].into_iter().counts()
        );

        let step4 = blink_with_counts(&step3);
        assert_eq!(
            step4,
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
                .into_iter()
                .counts()
        );

        let step5 = blink_with_counts(&step4);
        assert_eq!(
            step5,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
                .into_iter()
                .counts()
        );

        let step6 = blink_with_counts(&step5);
        assert_eq!(
            step6,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
            .into_iter()
            .counts()
        );
    }

    #[test]
    fn test_actual_problem_day11() {
        let data = fs::read_to_string("data/day11.txt").unwrap();
        let ns: Vec<_> = data
            .split_whitespace()
            .filter_map(|s| {
                println!("{s} {:?}", s.parse::<usize>());
                s.parse().ok()
            })
            .collect();

        let res = blink25(ns.clone());

        assert_eq!(res.len(), 186175);

        let res = blink75(ns);
        assert_eq!(res.values().sum::<usize>(), 220566831337810);
    }
}
