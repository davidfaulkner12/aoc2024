use std::collections::BTreeSet;

use crate::{
    day4::{char_matrix, TextPoint},
    day6::{explode_point_with_directions, Direction},
};

fn garden(cm: &Vec<Vec<u8>>) -> BTreeSet<BTreeSet<TextPoint>> {
    let mut points: Vec<TextPoint> = cm
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(j, _)| TextPoint { row: i, col: j })
        })
        .collect();

    fn dfs(
        cm: &Vec<Vec<u8>>,
        stack: &mut Vec<TextPoint>,
        visited: &mut BTreeSet<TextPoint>,
        complete: &mut BTreeSet<BTreeSet<TextPoint>>,
        current: &mut Option<(u8, BTreeSet<TextPoint>)>,
    ) {
        while let Some(next) = stack.pop() {
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);

            let next_char = cm[next.row][next.col];

            println!("{:?} {:?} {:?}", next, next_char, current);

            // If we are already following a character, is the next one in it?
            if let Some((c, p)) = current {
                if *c == next_char {
                    p.insert(next);
                } else {
                    complete.insert(p.clone());
                    *current = Some((next_char, BTreeSet::from([next])));
                }
            } else {
                *current = Some((next_char, BTreeSet::from([next])));
            }

            let cur_char = current.as_ref().unwrap().0;

            // Push nodes with the same char onto the stack
            explode_point_with_directions(
                next,
                TextPoint {
                    row: cm.len() - 1,
                    col: cm[0].len() - 1,
                },
                2,
            )
            .iter()
            .filter_map(|(d, v)| {
                if v.len() == 2
                    && (*d == Direction::N
                        || *d == Direction::E
                        || *d == Direction::S
                        || *d == Direction::W)
                    && cm[v[1].row][v[1].col] == cur_char
                {
                    Some(v[1])
                } else {
                    None
                }
            })
            .for_each(|p| {
                if !visited.contains(&p) {
                    stack.push(p);
                }
            });

            dfs(cm, stack, visited, complete, current)
        }
        if let Some((_, p)) = current {
            complete.insert(p.clone());
        }
    }

    let mut visited = BTreeSet::default();
    let mut complete = BTreeSet::default();
    let mut current = None;

    dfs(cm, &mut points, &mut visited, &mut complete, &mut current);

    complete
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "AAAA
BBCD
BBCC
EEEC";

    fn text_point(row: usize, col: usize) -> TextPoint {
        TextPoint { row, col }
    }

    #[test]
    fn test_build_a_garden() {
        let c = char_matrix(TEST_DATA);

        let expected = BTreeSet::from([
            BTreeSet::from([
                text_point(0, 0),
                text_point(0, 1),
                text_point(0, 2),
                text_point(0, 3),
            ]),
            BTreeSet::from([
                text_point(1, 0),
                text_point(1, 1),
                text_point(2, 0),
                text_point(2, 1),
            ]),
            BTreeSet::from([
                text_point(1, 2),
                text_point(2, 2),
                text_point(2, 3),
                text_point(3, 3),
            ]),
            BTreeSet::from([text_point(1, 3)]),
            BTreeSet::from([text_point(3, 0), text_point(3, 1), text_point(3, 2)]),
        ]);

        assert_eq!(garden(&c), expected);
    }
}
