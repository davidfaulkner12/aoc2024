use core::str;
use std::{cmp::min, collections::BTreeMap, fs};

use itertools::Itertools;

use crate::problem::Problem;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(pair: (usize, usize)) -> Self {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }

    fn with_x(&self, x: usize) -> Self {
        Point { x, y: self.y }
    }

    fn with_y(&self, y: usize) -> Self {
        Point { x: self.x, y }
    }
}

fn explode_point(p: Point, corner: Point, length: usize) -> Vec<Vec<Point>> {
    let min_x = p.x.checked_sub(length - 1).unwrap_or(0);
    let min_y = p.y.checked_sub(length - 1).unwrap_or(0);
    let max_x = min(p.x + length - 1, corner.x);
    let max_y = min(p.y + length - 1, corner.y);

    let with_x = |x| p.with_x(x);
    let with_y = |y| p.with_y(y);

    let mut vectors = Vec::with_capacity(8);
    // N
    vectors.push((min_x..=p.x).rev().map(with_x).collect());

    // NE
    vectors.push(
        ((min_x..=p.x).rev())
            .zip(p.y..=max_y)
            .map(Point::new)
            .collect(),
    );

    // E
    vectors.push((p.y..=max_y).map(with_y).collect());

    // SE
    vectors.push((p.x..=max_x).zip(p.y..=max_y).map(Point::new).collect());

    // S
    vectors.push((p.x..=max_x).map(with_x).collect());

    // SW
    vectors.push(
        (p.x..=max_x)
            .zip((min_y..=p.y).rev())
            .map(Point::new)
            .collect(),
    );

    // W
    vectors.push((min_y..=p.y).rev().map(with_y).collect());

    // NW
    vectors.push(
        (min_x..=p.x)
            .rev()
            .zip((min_y..=p.y).rev())
            .map(Point::new)
            .collect(),
    );

    vectors
}

fn extract_string_from_vector(puzzle: &Vec<&str>, vector: &[Point]) -> String {
    let bytes: Vec<_> = puzzle.iter().map(|s| s.as_bytes()).collect();
    str::from_utf8(&vector.iter().map(|p| bytes[p.x][p.y]).collect::<Vec<_>>())
        .unwrap()
        .to_string()
}

fn find_char_in_puzzle(puzzle: &Vec<&str>, c: u8) -> Vec<Point> {
    let bytes: Vec<_> = puzzle.iter().map(|s| s.as_bytes()).collect();
    bytes
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter().enumerate().filter_map(move |(y, maybe_c)| {
                if c == *maybe_c {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect()
}

#[derive(Default)]
pub struct Day4 {
    data: Vec<String>,
}

impl Day4 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day4.txt").unwrap();
        Day4::with_data(&data)
    }

    pub fn with_data(data: &str) -> Self {
        Day4 {
            data: data.lines().map(|s| s.to_owned()).collect(),
        }
    }

    fn prob1_inner(&mut self) -> usize {
        let corner = Point {
            x: self.data.len() - 1,
            y: self.data[0].len() - 1,
        };
        let p = self.data.iter().map(AsRef::as_ref).collect();
        let ps = find_char_in_puzzle(&p, b'X');
        let vs: Vec<_> = ps
            .iter()
            .flat_map(|p| explode_point(*p, corner, 4))
            .filter(|v| v.len() == 4)
            .collect();

        let words = vs.iter().map(|v| extract_string_from_vector(&p, v));

        //for (v, s) in vs.iter().zip(words.clone()) {
        //    println!("{:?} {:?}", v, s);
        //}

        words.filter(|s| *s == "XMAS").count()
    }

    fn prob2_inner(&mut self) -> usize {
        let corner = Point {
            x: self.data.len() - 1,
            y: self.data[0].len() - 1,
        };
        let p = self.data.iter().map(AsRef::as_ref).collect();
        let ps = find_char_in_puzzle(&p, b'M');
        let vs: Vec<_> = ps
            .iter()
            .map(|p| explode_point(*p, corner, 3))
            // Only diagonals
            .map(|vs| vec![vs[1].clone(), vs[3].clone(), vs[5].clone(), vs[7].clone()])
            .flatten()
            .filter(|v| v.len() == 3)
            .collect();

        let mut cs: BTreeMap<Point, Vec<Vec<Point>>> = BTreeMap::new();

        for v in vs {
            let k = v[1];
            if cs.contains_key(&k) {
                cs.get_mut(&k).unwrap().push(v.clone())
            } else {
                cs.insert(k.clone(), vec![v.clone()]);
            }
        }

        let mut count = 0;

        for (_, v) in cs.iter() {
            for combos in v.iter().combinations(2) {
                if (extract_string_from_vector(&p, combos[0]) == "MAS")
                    && (extract_string_from_vector(&p, combos[1]) == "MAS")
                {
                    count += 1
                }
            }
        }

        count
    }
}

impl Problem for Day4 {
    fn prob1(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob1_inner())
    }

    fn prob2(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob2_inner())
    }
}

#[cfg(test)]
mod tests {

    // use super::{get_pairs, get_pairs_stateful, Day4};
    use super::*;

    const TEST_DATA: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    fn make_vector(ps: &[(usize, usize)]) -> Vec<Point> {
        ps.iter().map(|(x, y)| Point { x: *x, y: *y }).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            explode_point(Point { x: 0, y: 0 }, Point { x: 3, y: 3 }, 4),
            vec![
                make_vector(&[(0, 0)]),
                make_vector(&[(0, 0)]),
                make_vector(&[(0, 0), (0, 1), (0, 2), (0, 3)]),
                make_vector(&[(0, 0), (1, 1), (2, 2), (3, 3)]),
                make_vector(&[(0, 0), (1, 0), (2, 0), (3, 0)]),
                make_vector(&[(0, 0)]),
                make_vector(&[(0, 0)]),
                make_vector(&[(0, 0)]),
            ]
        );
        assert_eq!(
            explode_point(Point { x: 3, y: 3 }, Point { x: 3, y: 3 }, 4),
            vec![
                make_vector(&[(3, 3), (2, 3), (1, 3), (0, 3)]),
                make_vector(&[(3, 3)]),
                make_vector(&[(3, 3)]),
                make_vector(&[(3, 3)]),
                make_vector(&[(3, 3)]),
                make_vector(&[(3, 3)]),
                make_vector(&[(3, 3), (3, 2), (3, 1), (3, 0)]),
                make_vector(&[(3, 3), (2, 2), (1, 1), (0, 0)]),
            ]
        );

        assert_eq!(
            explode_point(Point { x: 3, y: 0 }, Point { x: 3, y: 3 }, 4),
            vec![
                make_vector(&[(3, 0), (2, 0), (1, 0), (0, 0)]),
                make_vector(&[(3, 0), (2, 1), (1, 2), (0, 3)]),
                make_vector(&[(3, 0), (3, 1), (3, 2), (3, 3)]),
                make_vector(&[(3, 0)]),
                make_vector(&[(3, 0)]),
                make_vector(&[(3, 0)]),
                make_vector(&[(3, 0)]),
                make_vector(&[(3, 0)])
            ]
        );

        assert_eq!(
            explode_point(Point { x: 0, y: 3 }, Point { x: 3, y: 3 }, 4),
            vec![
                make_vector(&[(0, 3)]),
                make_vector(&[(0, 3)]),
                make_vector(&[(0, 3)]),
                make_vector(&[(0, 3)]),
                make_vector(&[(0, 3), (1, 3), (2, 3), (3, 3)]),
                make_vector(&[(0, 3), (1, 2), (2, 1), (3, 0)]),
                make_vector(&[(0, 3), (0, 2), (0, 1), (0, 0)]),
                make_vector(&[(0, 3)]),
            ]
        );
    }

    #[test]
    fn test_extract() {
        let v = make_vector(&[(0, 5), (0, 6), (0, 7), (0, 8)]);
        let p = TEST_DATA.lines().collect();
        assert_eq!(extract_string_from_vector(&p, &v), "XMAS");
    }

    #[test]
    fn test_find_char() {
        let p = TEST_DATA.lines().collect();
        assert_eq!(
            find_char_in_puzzle(&p, b'X'),
            make_vector(&[
                (0, 4),
                (0, 5),
                (1, 4),
                (2, 2),
                (2, 4),
                (3, 9),
                (4, 0),
                (4, 6),
                (5, 0),
                (5, 1),
                (5, 5),
                (5, 6),
                (6, 7),
                (7, 2),
                (8, 5),
                (9, 1),
                (9, 3),
                (9, 5),
                (9, 9)
            ])
        );
    }

    #[test]
    fn test_example_prob1() {
        let mut day4 = Day4::with_data(TEST_DATA);
        assert_eq!(day4.prob1_inner(), 18);
    }

    #[test]
    fn test_example_prob2() {
        let mut day4 = Day4::with_data(TEST_DATA);
        assert_eq!(day4.prob2_inner(), 9);
    }

    #[test]
    fn test_actual_problem() {
        let mut day4 = Day4::new();
        assert_eq!(day4.prob1_inner(), 2578);
        assert_eq!(day4.prob2_inner(), 1972);
    }
}
