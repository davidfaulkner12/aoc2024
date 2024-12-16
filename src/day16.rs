use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::day4::char_matrix;
use crate::day4::TextPoint;
use crate::day6::{explode_point_with_directions, Direction};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: TextPoint,
    direction: Direction,
}
// The priority queue depends on `Ord`.
// // Explicitly implement the trait so the queue becomes a min-heap
// // instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_edges(
    graph: &Vec<Vec<u8>>,
    position: TextPoint,
    direction: Direction,
) -> Vec<(usize, TextPoint, Direction)> {
    explode_point_with_directions(
        position,
        TextPoint {
            row: graph.len() - 1,
            col: graph[0].len() - 1,
        },
        2,
    )
    .iter()
    .filter_map(|(d, v)| {
        if v.len() != 2 || graph[v[1].row][v[1].col] == b'#' {
            None
        } else if *d == direction && graph[v[1].row][v[1].col] != b'#' {
            Some((1, v[1], *d))
        } else if *d == direction.rotate_right() {
            Some((1001, v[1], *d))
        } else if *d == direction.rotate_left() {
            Some((1001, v[1], *d))
        } else {
            None
        }
    })
    .collect()
}

// Start at `start` and use `dist` to track the current shortest distance
// // to each node. This implementation isn't memory-efficient as it may leave duplicate
// // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// // for a simpler implementation.
fn shortest_path(graph: &Vec<Vec<u8>>, start: TextPoint, goal: TextPoint) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<_, _> = graph
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, _)| (TextPoint { row: r, col: c }, usize::MAX))
        })
        .collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    *dist.get_mut(&start).unwrap() = 0;
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::E,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[&position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (next_cost, next_position, next_direction) in find_edges(graph, position, direction) {
            let next = State {
                cost: cost + next_cost,
                position: next_position,
                direction: next_direction,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[&next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist.get_mut(&next.position).unwrap() = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{fs, thread};

    use crate::day4::find_char_in_puzzle;

    use super::*;

    const TEST_DATA: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[test]
    fn test_find_edges() {
        let g = char_matrix(TEST_DATA);

        let res = find_edges(&g, TextPoint { row: 13, col: 1 }, Direction::E);

        assert_eq!(res.len(), 2);

        let (cost, position, direction) = res[1];
        assert_eq!(
            (cost, position, direction),
            (1, TextPoint { row: 13, col: 2 }, Direction::E)
        );
        let (cost, position, direction) = res[0];
        assert_eq!(
            (cost, position, direction),
            (7001, TextPoint { row: 12, col: 1 }, Direction::N)
        );
    }

    #[test]
    fn test_example_day16() {
        let g = char_matrix(TEST_DATA);
        let start = find_char_in_puzzle(&g, b'S')[0];
        let end = find_char_in_puzzle(&g, b'E')[0];

        let res = shortest_path(&g, start, end);

        assert_eq!(res, Some(7036));
    }

    const TEST_DATA_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    #[test]

    fn test_example_2_day16() {
        let g = char_matrix(TEST_DATA_2);
        let start = find_char_in_puzzle(&g, b'S')[0];
        let end = find_char_in_puzzle(&g, b'E')[0];

        let res = shortest_path(&g, start, end);

        assert_eq!(res, Some(11048));
    }

    #[test]
    fn test_actual_16() {
        let data = fs::read_to_string("data/day16.txt").unwrap();
        let g = char_matrix(&data);
        let start = find_char_in_puzzle(&g, b'S')[0];
        let end = find_char_in_puzzle(&g, b'E')[0];

        let res = shortest_path(&g, start, end);

        assert_eq!(res, Some(102460));
    }
}
