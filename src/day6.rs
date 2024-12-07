use day4::{explode_point, extract_string_from_vector, find_char_in_puzzle, TextPoint};
use enum_iterator::{all, Sequence};

use crate::day4;

#[derive(Copy, Clone, Debug, PartialEq, Sequence, Hash, Eq)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::NE => Direction::SE,
            Direction::E => Direction::S,
            Direction::SE => Direction::SW,
            Direction::S => Direction::W,
            Direction::SW => Direction::NW,
            Direction::W => Direction::N,
            Direction::NW => Direction::NE,
        }
    }
}

fn explode_point_with_directions(
    p: TextPoint,
    corner: TextPoint,
    length: usize,
) -> Vec<(Direction, Vec<TextPoint>)> {
    let v = explode_point(p, corner, length);

    all::<Direction>().zip(v).collect()
}

#[derive(Debug, Clone)]
struct GuardMap {
    guard_map: Vec<String>,
    guard: (Direction, Option<TextPoint>),
}

impl GuardMap {
    fn tick(&mut self) -> Option<TextPoint> {
        let corner = TextPoint {
            row: self.guard_map.len() - 1,
            col: self.guard_map[0].len() - 1,
        };

        let p = self.guard_map.iter().map(AsRef::as_ref).collect();
        if let Some(guard_point) = self.guard.1 {
            let all_moves = explode_point_with_directions(guard_point, corner, 2);

            // I kind of wanted to make this recursive, but decided to loop instead
            for _ in 0..4 {
                let v = all_moves
                    .iter()
                    .filter(|(d, _)| d == &self.guard.0)
                    .nth(0)
                    .unwrap();
                let s = extract_string_from_vector(&p, &v.1);
                if s == "." {
                    self.guard.1 = None;
                    break;
                }
                if s == ".." {
                    self.guard.1 = Some(v.1[1]);
                    break;
                }
                self.guard.0 = self.guard.0.rotate_right();
            }
        }

        self.guard.1
    }
}

fn parse(data: &str) -> GuardMap {
    let data: Vec<String> = data.lines().map(|s| s.to_owned()).collect();

    // This is awkward but matching the caller convention in day 4
    let p = data.iter().map(AsRef::as_ref).collect();
    let g = find_char_in_puzzle(&p, b'^')[0];

    let data = replace_char_in_puzzle(&data, g, ".");

    GuardMap {
        guard_map: data,
        guard: (Direction::N, Some(g)),
    }
}

fn replace_char_in_puzzle(puzzle: &[String], p: TextPoint, new: &str) -> Vec<String> {
    let mut puzzle = puzzle.clone().to_vec();
    let mut s = puzzle[p.row].clone();
    s.replace_range(p.col..=p.col, new);
    puzzle[p.row] = s;
    puzzle
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use super::*;

    const TEST_DATA: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_explode_with_directions() {
        let res = explode_point_with_directions(
            TextPoint { row: 3, col: 2 },
            TextPoint { row: 8, col: 8 },
            2,
        );
        assert_eq!(
            res[0],
            (
                Direction::N,
                vec![TextPoint { row: 3, col: 2 }, TextPoint { row: 2, col: 2 }]
            )
        );
    }

    #[test]
    fn test_parse() {
        let res = parse(TEST_DATA);

        assert_eq!(
            res.guard,
            (Direction::N, Some(TextPoint { row: 6, col: 4 }))
        );

        assert_eq!(res.guard_map[6].as_bytes()[4], b'.');
    }

    #[test]
    fn test_tick() {
        let mut guard_map = parse(TEST_DATA);

        guard_map.tick();

        assert_eq!(
            guard_map.guard,
            (Direction::N, Some(TextPoint { row: 5, col: 4 }))
        );

        guard_map.tick();
        guard_map.tick();
        guard_map.tick();
        guard_map.tick();
        guard_map.tick();

        assert_eq!(
            guard_map.guard,
            (Direction::E, Some(TextPoint { row: 1, col: 5 }))
        );
    }

    //#[test]
    //fn actual() {
    //    let data = fs::read_to_string("data/day6.txt").unwrap();
    //    //let data = TEST_DATA;
    //    let mut guard_map = parse(&data);
    //    let mut visited: HashSet<TextPoint> = HashSet::new();
    //
    //    while let Some(p) = guard_map.guard.1 {
    //        visited.insert(p);
    //        guard_map.tick();
    //    }
    //
    //    assert_eq!(visited.len(), 4711);
    //    //assert_eq!(visited.len(), 41);
    //
    //    // Now for part 2
    //    let mut count = 0;
    //    let guard_map_2 = parse(&data);
    //    let start = guard_map_2.guard.1.unwrap();
    //
    //    for o in visited.iter() {
    //        // Special case the starting point
    //        if start == *o {
    //            continue;
    //        }
    //        println!("Considering point {:?}", o);
    //        let mut visited: HashSet<(Direction, TextPoint)> = HashSet::new();
    //        let mut guard_map = GuardMap {
    //            guard_map: replace_char_in_puzzle(&guard_map_2.guard_map, *o, "O"),
    //            guard: guard_map_2.guard,
    //        };
    //        while let Some(p) = guard_map.guard.1 {
    //            let maybe_loop = (guard_map.guard.0, p);
    //            if visited.contains(&maybe_loop) {
    //                count += 1;
    //                break;
    //            }
    //            visited.insert(maybe_loop);
    //            guard_map.tick();
    //        }
    //    }
    //
    //    assert_eq!(count, 1562);
    //}
}
