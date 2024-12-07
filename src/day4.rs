use core::str;
use std::{
    cmp::min,
    collections::{BTreeMap, HashMap},
    fs,
};

use itertools::Itertools;
use linkme::distributed_slice;

use crate::problem::{Problem, PROBLEMS};

// Intuitively we use row/col because that's how we index Vec<String>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct TextPoint {
    pub row: usize,
    pub col: usize,
}

impl TextPoint {
    fn new(pair: (usize, usize)) -> Self {
        TextPoint {
            row: pair.0,
            col: pair.1,
        }
    }

    fn with_row(&self, row: usize) -> Self {
        TextPoint { row, col: self.col }
    }

    fn with_col(&self, col: usize) -> Self {
        TextPoint { row: self.row, col }
    }
}

pub fn explode_point(p: TextPoint, corner: TextPoint, length: usize) -> Vec<Vec<TextPoint>> {
    let min_row = p.row.checked_sub(length - 1).unwrap_or(0);
    let min_col = p.col.checked_sub(length - 1).unwrap_or(0);
    let max_row = min(p.row + length - 1, corner.row);
    let max_col = min(p.col + length - 1, corner.col);

    let with_row = |row| p.with_row(row);
    let with_col = |col| p.with_col(col);

    let mut vectors = Vec::with_capacity(8);
    // N
    vectors.push((min_row..=p.row).rev().map(with_row).collect());

    // NE
    vectors.push(
        ((min_row..=p.row).rev())
            .zip(p.col..=max_col)
            .map(TextPoint::new)
            .collect(),
    );

    // E
    vectors.push((p.col..=max_col).map(with_col).collect());

    // SE
    vectors.push(
        (p.row..=max_row)
            .zip(p.col..=max_col)
            .map(TextPoint::new)
            .collect(),
    );

    // S
    vectors.push((p.row..=max_row).map(with_row).collect());

    // SW
    vectors.push(
        (p.row..=max_row)
            .zip((min_col..=p.col).rev())
            .map(TextPoint::new)
            .collect(),
    );

    // W
    vectors.push((min_col..=p.col).rev().map(with_col).collect());

    // NW
    vectors.push(
        (min_row..=p.row)
            .rev()
            .zip((min_col..=p.col).rev())
            .map(TextPoint::new)
            .collect(),
    );

    vectors
}

pub fn extract_string_from_vector(puzzle: &Vec<&str>, vector: &[TextPoint]) -> String {
    let bytes: Vec<_> = puzzle.iter().map(|s| s.as_bytes()).collect();
    str::from_utf8(
        &vector
            .iter()
            .map(|p| bytes[p.row][p.col])
            .collect::<Vec<_>>(),
    )
    .unwrap()
    .to_string()
}

pub fn find_char_in_puzzle(puzzle: &Vec<&str>, c: u8) -> Vec<TextPoint> {
    let bytes: Vec<_> = puzzle.iter().map(|s| s.as_bytes()).collect();
    bytes
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, maybe_c)| {
                if c == *maybe_c {
                    Some(TextPoint { row, col })
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
        let corner = TextPoint {
            row: self.data.len() - 1,
            col: self.data[0].len() - 1,
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
        let corner = TextPoint {
            row: self.data.len() - 1,
            col: self.data[0].len() - 1,
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

        let mut cs: BTreeMap<TextPoint, Vec<Vec<TextPoint>>> = BTreeMap::new();

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

#[distributed_slice(PROBLEMS)]
fn register_day(p: &mut HashMap<String, fn() -> Box<dyn Problem>>) {
    p.insert("day4".to_owned(), || Box::new(Day4::new()));
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

    fn make_vector(ps: &[(usize, usize)]) -> Vec<TextPoint> {
        ps.iter()
            .map(|(row, col)| TextPoint {
                row: *row,
                col: *col,
            })
            .collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            explode_point(
                TextPoint { row: 0, col: 0 },
                TextPoint { row: 3, col: 3 },
                4
            ),
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
            explode_point(
                TextPoint { row: 3, col: 3 },
                TextPoint { row: 3, col: 3 },
                4
            ),
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
            explode_point(
                TextPoint { row: 3, col: 0 },
                TextPoint { row: 3, col: 3 },
                4
            ),
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
            explode_point(
                TextPoint { row: 0, col: 3 },
                TextPoint { row: 3, col: 3 },
                4
            ),
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
