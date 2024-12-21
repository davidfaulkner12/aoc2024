use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::day4::char_matrix;
use crate::day4::TextPoint;
use crate::day6::{explode_point_with_directions, Direction};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: TextPoint,
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

fn find_edges(graph: &Vec<Vec<u8>>, position: TextPoint) -> Vec<(usize, TextPoint)> {
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
        } else if *d == Direction::N
            || *d == Direction::E
            || *d == Direction::S
            || *d == Direction::W
        {
            Some((1, v[1]))
        } else {
            None
        }
    })
    .collect()
}

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(graph: &Vec<Vec<u8>>) -> Option<usize> {
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

    let start = TextPoint { row: 0, col: 0 };
    let goal = TextPoint {
        row: graph.len() - 1,
        col: graph[0].len() - 1,
    };

    // We're at `start`, with a zero cost
    *dist.get_mut(&start).unwrap() = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
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
        for (next_cost, next_position) in find_edges(graph, position) {
            let next = State {
                cost: cost + next_cost,
                position: next_position,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[&next_position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist.get_mut(&next_position).unwrap() = next.cost;
            }
        }
        //print_map_with_costs(&graph, &dist);
        //println!("=======");
    }

    // Goal not reachable
    None
}

fn parse(data: &str) -> Vec<(usize, usize)> {
    data.lines()
        .map(|line| {
            let mut split = line.split(",");
            // Yes this is weird but the problem uses x,y
            let col = str::parse(split.next().unwrap()).unwrap();
            let row = str::parse(split.next().unwrap()).unwrap();
            (row, col)
        })
        .collect()
}

fn build_graph(rows: usize, cols: usize, blobs: &[(usize, usize)]) -> Vec<Vec<u8>> {
    let mut res = Vec::with_capacity(rows);

    for r in 0..rows {
        res.push(Vec::with_capacity(cols));
        for c in 0..cols {
            if blobs.contains(&(r, c)) {
                res[r].push(b'#');
            } else {
                res[r].push(b'.');
            }
        }
    }

    res
}

fn print_map_with_costs(g: &Vec<Vec<u8>>, dist: &HashMap<TextPoint, usize>) {
    for k in 0..g.len() {
        for j in 0..g[0].len() {
            let c = dist[&TextPoint { row: k, col: j }];
            if g[k][j] == b'#' {
                print!("#  ");
            } else if c != usize::MAX {
                print!("{:>2} ", c);
            } else {
                print!(".  ");
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{fs, thread};

    use super::*;

    const TEST_DATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_example_18() {
        let bs = parse(TEST_DATA);
        let g = build_graph(7, 7, &bs.as_slice()[0..11]);

        let res = shortest_path(&g);

        assert_eq!(res, Some(22));
    }

    #[test]
    fn test_actual_18() {
        let data = fs::read_to_string("data/day18.txt").unwrap();
        let bs = parse(&data);
        let g = build_graph(71, 71, &bs.as_slice()[0..1024]);

        let res = shortest_path(&g);

        assert_eq!(res, Some(270));
    }

    #[test]
    fn test_example_18_2() {
        //let data = TEST_DATA;
        let data = fs::read_to_string("data/day18.txt").unwrap();
        let bs = parse(&data);

        let mut res = 0;

        for i in 1024..3000 {
            //for i in 12..3000 {
            //let g = build_graph(7, 7, &bs.as_slice()[0..i]);
            if i % 250 == 0 {
                println!("{}", i);
            }
            let g = build_graph(71, 71, &bs.as_slice()[0..i]);
            let path = shortest_path(&g);
            if path.is_none() {
                res = i;
                break;
            }
        }

        // Remember these are inverted
        assert_eq!(bs[res - 1], (40, 51));
    }
}
