use std::collections::HashSet;

use crate::{
    day4::{explode_point, find_char_in_puzzle, TextPoint},
    day6::{explode_point_with_directions, Direction},
};

fn climb_trail(map: &Vec<Vec<u8>>, start: TextPoint) -> HashSet<(TextPoint, TextPoint)> {
    let corner = TextPoint {
        row: map.len() - 1,
        col: map[0].len() - 1,
    };

    let mut s = vec![start];

    let mut routes = HashSet::new();

    while let Some(p) = s.pop() {
        let c = map[p.row][p.col];
        if c == b'9' {
            routes.insert((start, p));
            continue;
        }

        let all_moves: Vec<_> = explode_point_with_directions(p, corner, 2)
            .into_iter()
            .filter(|(d, v)| {
                (v.len() == 2)
                    && (*d == Direction::N
                        || *d == Direction::E
                        || *d == Direction::S
                        || *d == Direction::W)
            })
            .collect();

        for (_, maybe_next) in all_moves {
            println!("Considering {maybe_next:?}");
            let maybe_next_p = maybe_next[1];
            let maybe_next_c = map[maybe_next_p.row][maybe_next_p.col];
            if maybe_next_c.saturating_sub(c) == 1 {
                println!("Adding {maybe_next_p:?}");
                s.push(maybe_next_p);
            }
        }
    }

    routes
}

fn climb_all_trail(map: &Vec<Vec<u8>>, start: TextPoint) -> Vec<(TextPoint, TextPoint)> {
    let corner = TextPoint {
        row: map.len() - 1,
        col: map[0].len() - 1,
    };

    let mut s = vec![start];

    let mut routes = Vec::new();

    while let Some(p) = s.pop() {
        let c = map[p.row][p.col];
        if c == b'9' {
            routes.push((start, p));
            continue;
        }

        let all_moves: Vec<_> = explode_point_with_directions(p, corner, 2)
            .into_iter()
            .filter(|(d, v)| {
                (v.len() == 2)
                    && (*d == Direction::N
                        || *d == Direction::E
                        || *d == Direction::S
                        || *d == Direction::W)
            })
            .collect();

        for (_, maybe_next) in all_moves {
            println!("Considering {maybe_next:?}");
            let maybe_next_p = maybe_next[1];
            let maybe_next_c = map[maybe_next_p.row][maybe_next_p.col];
            if maybe_next_c.saturating_sub(c) == 1 {
                println!("Adding {maybe_next_p:?}");
                s.push(maybe_next_p);
            }
        }
    }

    routes
}

fn find_unique_trails(map: &Vec<Vec<u8>>) -> Vec<(TextPoint, usize)> {
    let zeros = find_char_in_puzzle(&map, b'0');

    zeros
        .into_iter()
        .map(|p| (p, climb_trail(&map, p).len()))
        .collect()
}

fn find_all_trails(map: &Vec<Vec<u8>>) -> Vec<(TextPoint, usize)> {
    let zeros = find_char_in_puzzle(&map, b'0');

    zeros
        .into_iter()
        .map(|p| (p, climb_all_trail(&map, p).len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::day4::char_matrix;

    use super::*;

    const TEST_DATA: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_basic_path_finding() {
        let c = char_matrix(TEST_DATA);

        let trails = climb_trail(&c, TextPoint { row: 0, col: 2 });
        assert_eq!(trails.len(), 5);
    }

    #[test]
    fn test_sample() {
        let c = char_matrix(TEST_DATA);
        let res = find_all_trails(&c);

        assert_eq!(res.into_iter().map(|(_, n)| n).sum::<usize>(), 36);
    }

    #[test]
    fn test_sample_prob2_day10() {
        let data = TEST_DATA;
        let c = char_matrix(&data);

        let res = find_all_trails(&c);
        assert_eq!(res.into_iter().map(|(_, n)| n).sum::<usize>(), 81);
    }

    #[test]
    fn test_problem_day10() {
        let data = fs::read_to_string("data/day10.txt").unwrap();
        let c = char_matrix(&data);

        let res = find_unique_trails(&c);
        assert_eq!(res.into_iter().map(|(_, n)| n).sum::<usize>(), 796);

        let res = find_all_trails(&c);
        assert_eq!(res.into_iter().map(|(_, n)| n).sum::<usize>(), 1942);
    }
}
