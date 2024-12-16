use day4::{explode_point, extract_string_from_vector, find_char_in_puzzle, TextPoint};
use enum_iterator::{all, Sequence};

use crate::day4::{self, char_matrix};

#[derive(Copy, Clone, Debug, PartialEq, Sequence, Hash, Eq)]
pub enum Direction {
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
    pub fn rotate_right(&self) -> Self {
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
    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::NW => Direction::SW,
            Direction::W => Direction::S,
            Direction::SW => Direction::SE,
            Direction::S => Direction::E,
            Direction::SE => Direction::NE,
            Direction::E => Direction::N,
            Direction::NE => Direction::NW,
        }
    }
}

pub fn explode_point_with_directions(
    p: TextPoint,
    corner: TextPoint,
    length: usize,
) -> Vec<(Direction, Vec<TextPoint>)> {
    let v = explode_point(p, corner, length);

    all::<Direction>().zip(v).collect()
}

#[derive(Debug, Clone)]
struct GuardMap {
    guard_map: Vec<Vec<u8>>,
    guard: (Direction, Option<TextPoint>),
}

impl GuardMap {
    fn tick(&mut self) -> Option<TextPoint> {
        let corner = TextPoint {
            row: self.guard_map.len() - 1,
            col: self.guard_map[0].len() - 1,
        };

        if let Some(guard_point) = self.guard.1 {
            let all_moves = explode_point_with_directions(guard_point, corner, 2);

            // I kind of wanted to make this recursive, but decided to loop instead
            for _ in 0..4 {
                let v = all_moves
                    .iter()
                    .filter(|(d, _)| d == &self.guard.0)
                    .nth(0)
                    .unwrap();
                let s = extract_string_from_vector(&self.guard_map, &v.1);
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
    let data: Vec<Vec<u8>> = char_matrix(data);

    let g = find_char_in_puzzle(&data, b'^')[0];

    let data = replace_char_in_puzzle(&data, g, b'.');

    GuardMap {
        guard_map: data,
        guard: (Direction::N, Some(g)),
    }
}

fn replace_char_in_puzzle(puzzle: &Vec<Vec<u8>>, p: TextPoint, new: u8) -> Vec<Vec<u8>> {
    let mut puzzle = puzzle.clone().to_vec();
    puzzle[p.row][p.col] = new;
    puzzle
}

#[cfg(test)]
mod tests {
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

        assert_eq!(res.guard_map[6][4], b'.');
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
