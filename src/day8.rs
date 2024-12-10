use crate::day4::TextPoint;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct TextDifference {
    row: isize,
    col: isize,
}

impl TextDifference {
    fn apply(&self, other: TextPoint) -> Option<TextPoint> {
        let row_diff = other.row.checked_add_signed(self.row);
        let col_diff = other.col.checked_add_signed(self.col);

        if let (Some(row_diff), Some(col_diff)) = (row_diff, col_diff) {
            Some(TextPoint {
                row: row_diff,
                col: col_diff,
            })
        } else {
            None
        }
    }

    fn apply_with_corner(&self, other: TextPoint, corner: TextPoint) -> Option<TextPoint> {
        let new_point = self.apply(other);
        match new_point {
            Some(TextPoint { row, col }) if row <= corner.row && col <= corner.col => new_point,
            _ => None,
        }
    }

    fn flip(&self) -> TextDifference {
        TextDifference {
            row: -self.row,
            col: -self.col,
        }
    }
}

fn point_distance(a: TextPoint, b: TextPoint) -> TextDifference {
    TextDifference {
        row: (b.row as isize) - (a.row as isize),
        col: (b.col as isize) - (a.col as isize),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        convert::identity,
        fs,
    };

    use crate::day4::char_matrix;

    use super::*;

    const TEST_DATA: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_point_distance() {
        let res = point_distance(TextPoint { row: 0, col: 0 }, TextPoint { row: 1, col: 1 });
        assert_eq!(res, TextDifference { row: 1, col: 1 });

        let res = res.apply(TextPoint { row: 0, col: 0 });
        assert_eq!(res, Some(TextPoint { row: 1, col: 1 }));

        let res = point_distance(TextPoint { row: 0, col: 1 }, TextPoint { row: 1, col: 0 });
        assert_eq!(res, TextDifference { row: 1, col: -1 });

        let res1 = res.apply(TextPoint { row: 0, col: 1 });
        assert_eq!(res1, Some(TextPoint { row: 1, col: 0 }));

        let res = res.apply(TextPoint { row: 0, col: 0 });
        assert_eq!(res, None);

        let res = point_distance(TextPoint { row: 0, col: 1 }, TextPoint { row: 2, col: 2 });
        let res = res.apply_with_corner(TextPoint { row: 2, col: 2 }, TextPoint { row: 3, col: 3 });
        assert_eq!(res, None);
    }

    #[test]
    fn test_flip() {
        let res = point_distance(TextPoint { row: 0, col: 1 }, TextPoint { row: 1, col: 0 });
        assert_eq!(
            res.apply(TextPoint { row: 0, col: 1 }),
            Some(TextPoint { row: 1, col: 0 })
        );
        assert_eq!(
            res.flip().apply(TextPoint { row: 1, col: 0 }),
            Some(TextPoint { row: 0, col: 1 })
        );
    }

    #[test]
    fn test_parse_sample() {
        //let board = char_matrix(TEST_DATA);

        let data = fs::read_to_string("data/day8.txt").unwrap();
        let board = char_matrix(&data);

        let mut coordinates: HashMap<u8, Vec<TextPoint>> = HashMap::new();
        let mut antennas: HashSet<TextPoint> = HashSet::new();

        for (i, v) in board.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                if *c == b'.' {
                    continue;
                }
                let point = TextPoint { row: i, col: j };
                antennas.insert(point);
                if let Some(ps) = coordinates.get_mut(&c) {
                    ps.push(point);
                } else {
                    coordinates.insert(*c, vec![point]);
                }
            }
        }

        //assert_eq!(
        //    coordinates.get(&b'A'),
        //    Some(&vec![
        //        TextPoint { row: 5, col: 6 },
        //        TextPoint { row: 8, col: 8 },
        //        TextPoint { row: 9, col: 9 }
        //    ])
        //);

        let corner = TextPoint {
            row: board.len() - 1,
            col: board[0].len() - 1,
        };

        let antinodes: HashSet<_> = coordinates
            .iter()
            .flat_map(|(_, v)| {
                v.iter()
                    .combinations(2)
                    .flat_map(|lr| {
                        let (l, r) = (lr[0], lr[1]);
                        let diff = point_distance(*l, *r);
                        vec![
                            diff.flip().apply_with_corner(*l, corner),
                            diff.apply_with_corner(*r, corner),
                        ]
                    })
                    .collect::<Vec<_>>()
            })
            .filter_map(identity)
            .collect();

        //assert_eq!(antinodes.len(), 14);
        assert_eq!(antinodes.len(), 259);

        let even_more_antinodes: HashSet<_> = coordinates
            .iter()
            .flat_map(|(_, v)| {
                let mut results = Vec::default();
                for lr in v.iter().combinations(2) {
                    let (l, r) = (lr[0], lr[1]);
                    let diff = point_distance(*l, *r);
                    let mut cur_point = Some(*r);
                    while let Some(point) = cur_point {
                        results.push(point);
                        cur_point = diff.apply_with_corner(point, corner);
                    }
                    cur_point = Some(*l);
                    while let Some(point) = cur_point {
                        results.push(point);
                        cur_point = diff.flip().apply_with_corner(point, corner);
                    }
                }
                results
            })
            .collect();

        //assert_eq!(even_more_antinodes.len(), 34);
        assert_eq!(even_more_antinodes.len(), 927);
    }
}
