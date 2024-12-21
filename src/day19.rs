use std::{collections::HashMap, convert::identity};

use itertools::{EitherOrBoth, Itertools};

fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut split = data.split("\n\n");

    let towels = split
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.as_bytes().to_vec())
        .collect();

    let patterns = split
        .next()
        .unwrap()
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    (towels, patterns)
}

fn parse_as_string(data: &str) -> (Vec<String>, Vec<String>) {
    let mut split = data.split("\n\n");

    let towels = split
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    let patterns = split
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    (towels, patterns)
}

fn search(towels: &Vec<Vec<u8>>, pattern: &Vec<u8>, path: &Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>> {
    let current = path.iter().flat_map(identity);
    let current_len: usize = path.iter().map(|c| c.len()).sum();

    let is_match = current.zip(pattern).map(|(a, b)| a == b).all(identity);

    if !is_match || current_len > pattern.len() {
        return None;
    }

    if current_len == pattern.len() {
        return Some(path.clone());
    }

    let target = pattern[current_len];

    for t in towels {
        if t[0] == target {
            let mut path = path.clone();
            path.push(t.clone());
            if let Some(res) = search(towels, pattern, &path) {
                return Some(res);
            }
        }
    }

    None
}

fn search_combos(
    towels: &Vec<String>,
    pattern: String,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if pattern.len() == 0 {
        1
    } else if let Some(res) = cache.get(&pattern) {
        *res
    } else {
        let res = towels
            .iter()
            .filter(|t| t.len() <= pattern.len() && pattern.starts_with(*t))
            .map(|t| {
                search_combos(
                    &towels.clone(),
                    pattern.split_at(t.len()).1.to_string(),
                    cache,
                )
            })
            .sum();
        cache.insert(pattern, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{fs, thread};

    use super::*;

    const TEST_DATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_parse() {
        let (towels, _patterns) = parse(TEST_DATA);

        assert_eq!(
            towels,
            vec![
                vec![b'r'],
                vec![b'w', b'r'],
                vec![b'b'],
                vec![b'g'],
                vec![b'b', b'w', b'u'],
                vec![b'r', b'b'],
                vec![b'g', b'b'],
                vec![b'b', b'r']
            ]
        );
    }

    #[test]
    fn test_search_one() {
        let (towels, patterns) = parse(TEST_DATA);

        let res = search(&towels, &patterns[0], &vec![]);

        assert_eq!(
            res,
            Some(vec![
                towels[2].clone(),
                towels[0].clone(),
                towels[1].clone(),
                towels[0].clone()
            ])
        );
    }

    #[test]
    fn test_search_all() {
        let (towels, patterns) = parse(TEST_DATA);

        let res = patterns
            .iter()
            .filter_map(|p| search(&towels, p, &vec![]))
            .count();

        assert_eq!(res, 6);
    }

    #[test]
    fn test_problem_day19() {
        let data = fs::read_to_string("data/day19.txt").unwrap();
        let (towels, patterns) = parse(&data);

        let res = patterns
            .iter()
            .filter_map(|p| search(&towels, p, &vec![]))
            .count();

        assert_eq!(res, 236);
    }

    #[test]
    fn test_search_combos() {
        let (towels, patterns) = parse_as_string(TEST_DATA);

        let mut cache = HashMap::new();

        let res = search_combos(&towels, patterns[0].clone(), &mut cache);
        assert_eq!(res, 2);

        let res: usize = patterns
            .into_iter()
            .map(|p| search_combos(&towels, p, &mut cache))
            .sum();

        assert_eq!(res, 16);
    }

    #[test]
    fn test_problem_day19_part2() {
        let data = fs::read_to_string("data/day19.txt").unwrap();
        let (towels, patterns) = parse_as_string(&data);

        let mut cache = HashMap::new();

        let res: usize = patterns
            .into_iter()
            .map(|p| search_combos(&towels, p, &mut cache))
            .sum();

        assert_eq!(res, 643685981770598);
    }
}
