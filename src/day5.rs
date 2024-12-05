use std::{
    collections::{HashMap, HashSet},
    convert::identity,
    fs,
};

use linkme::distributed_slice;

use crate::problem::{Problem, PROBLEMS};

// Graph implementation from:
// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
// and modified to carry a value
type NodeValue = usize;
type NodeIndex = usize;
type EdgeIndex = usize;

#[derive(Debug, Copy, Clone)]
pub struct NodeData {
    value: NodeValue,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, Copy, Clone)]
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Debug, Default, Clone)]
struct Graph {
    index: HashMap<NodeValue, NodeIndex>,
    edge_set: HashSet<(NodeValue, NodeValue)>,
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Graph {
    pub fn add_node(&mut self, value: NodeValue) -> NodeIndex {
        if let Some(index) = self.index.get(&value) {
            return *index;
        }
        let index = self.nodes.len();
        self.index.insert(value, index);
        self.nodes.push(NodeData {
            value,
            first_outgoing_edge: None,
        });
        index
    }

    pub fn add_edge_by_value(&mut self, source: NodeValue, target: NodeValue) {
        // If we already have a *direct* link, we don't need to do anything
        if self.edge_set.contains(&(source, target)) {
            return;
        }
        let source_index = *self.index.get(&source).unwrap();
        let target_index = *self.index.get(&target).unwrap();
        self.edge_set.insert((source, target));
        self.add_edge(source_index, target_index);
    }

    // Note that this method does *NOT* check for duplicates
    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

fn parse_rules(rules: &str) -> Graph {
    let mut g = Graph::default();

    for line in rules.lines() {
        let mut ns = line.split("|");

        let s: NodeValue = ns.next().unwrap().parse().unwrap();
        let t: NodeValue = ns.next().unwrap().parse().unwrap();

        g.add_node(s);
        g.add_node(t);
        g.add_edge_by_value(s, t);
    }

    g
}

fn parse_updates(updates: &str) -> Vec<Vec<NodeValue>> {
    updates
        .lines()
        .map(|line| line.split(",").map(|n| str::parse(n).unwrap()).collect())
        .collect()
}

fn extend_graph(g: &Graph, values: &[NodeValue]) -> Graph {
    let mut g = g.clone();
    let mut prev = values[0];
    g.add_node(prev);
    for next in &values[1..] {
        g.add_node(*next);
        g.add_edge_by_value(prev, *next);
        prev = *next;
    }
    g
}

fn extend_graph_with_values<'a>(g: &'a Graph, values: &'a [NodeValue]) -> (Graph, &'a [NodeValue]) {
    (extend_graph(g, values), values)
}

// Directly off wikipedia, modified with a filter: https://en.wikipedia.org/wiki/Cycle_(graph_theory)
fn contains_cycle(g: &Graph, filter: &HashSet<NodeIndex>) -> bool {
    let mut visited = HashSet::<NodeIndex>::new();
    let mut finished = HashSet::new();

    fn dfs(
        g: &Graph,
        filter: &HashSet<NodeIndex>,
        v: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        finished: &mut HashSet<NodeIndex>,
    ) -> bool {
        if finished.contains(&v) || !filter.contains(&v) {
            return false;
        } else if visited.contains(&v) {
            return true;
        }
        visited.insert(v);
        if g.successors(v)
            .map(|v| dfs(g, filter, v, visited, finished))
            .any(identity)
        {
            return true;
        }
        finished.insert(v);
        false
    }

    g.nodes
        .iter()
        .enumerate()
        .map(|(v, _)| dfs(g, filter, v, &mut visited, &mut finished))
        .any(identity)
}

// Again, taken directly from Wikipedia. We ignore the cycle check
fn topological_sort(g: &Graph, nodes: &[NodeIndex]) -> Vec<NodeValue> {
    let mut l = Vec::with_capacity(nodes.len());
    // "Permanent mark"
    let mut p: HashSet<NodeIndex> = HashSet::new();
    // "Temporary mark"
    let mut t: HashSet<NodeIndex> = HashSet::new();

    let nodes_index: Vec<_> = nodes.iter().map(|n| *g.index.get(n).unwrap()).collect();

    fn visit(
        g: &Graph,
        nodes: &[NodeIndex],
        l: &mut Vec<NodeIndex>,
        p: &mut HashSet<NodeIndex>,
        t: &mut HashSet<NodeIndex>,
        n: NodeIndex,
    ) {
        if p.contains(&n) {
            return;
        }

        // Yes, this means we have a cycle, but we aren't going to worry about it
        if t.contains(&n) {
            return;
        }

        t.insert(n);

        for m in g.successors(n).filter(|x| nodes.contains(x)) {
            visit(g, nodes, l, p, t, m)
        }

        p.insert(n);
        l.push(n);
    }

    for n in nodes_index.iter() {
        // If we've already pushed this node, skip it.
        if p.contains(n) {
            continue;
        }
        visit(&g, &nodes_index, &mut l, &mut p, &mut t, *n)
    }

    l.reverse();

    l.into_iter()
        .map(|n| g.nodes[n].value)
        .filter(|n| nodes.contains(n))
        .collect()
}

#[derive(Default)]
pub struct Day5Part1 {
    rules: Graph,
    updates: Vec<Vec<NodeIndex>>,
}

#[derive(Default)]
pub struct Day5Part2 {
    rules: Graph,
    invalid: Vec<Vec<NodeIndex>>,
}

impl Day5Part1 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day5.txt").unwrap();
        Day5Part1::with_data(&data)
    }

    pub fn with_data(data: &str) -> Self {
        let parsed_data: Vec<_> = data.split("\n\n").collect();

        Day5Part1 {
            rules: parse_rules(parsed_data[0]),
            updates: parse_updates(parsed_data[1]),
        }
    }

    fn prob1_inner(self) -> (Day5Part2, usize) {
        let (valid, invalid): (Vec<_>, Vec<_>) = self
            .updates
            .iter()
            .map(|values| extend_graph_with_values(&self.rules, values))
            .partition(|(g, values)| {
                let filter: HashSet<NodeIndex> =
                    HashSet::from_iter(values.iter().map(|v| *g.index.get(&v).unwrap()));
                !contains_cycle(&g, &filter)
            });

        let invalid = invalid.iter().map(|(_, v)| v.to_vec()).collect();

        let res = valid
            .iter()
            //.inspect(|(_, values)| println!("{:?}", values))
            .map(|(_, values)| values[values.len() / 2])
            //.inspect(|value| println!("{:?}", value))
            .sum();

        (
            Day5Part2 {
                rules: self.rules,
                invalid,
            },
            res,
        )
    }
}

impl Day5Part2 {
    fn prob2_inner(&self) -> usize {
        self.invalid
            .iter()
            .map(|v| topological_sort(&self.rules, v))
            .map(|v| v[v.len() / 2])
            .sum()
    }
}

#[derive(Default)]
struct Day5 {
    part1: Option<Day5Part1>,
    part2: Option<Day5Part2>,
}

impl Day5 {
    fn new() -> Self {
        Day5 {
            part1: Some(Day5Part1::new()),
            part2: None,
        }
    }

    fn prob1_inner(&mut self) -> usize {
        let part1 = self.part1.take().unwrap();
        let (part2, res) = part1.prob1_inner();
        self.part2 = Some(part2);
        res
    }

    fn prob2_inner(&mut self) -> usize {
        self.part2.as_ref().unwrap().prob2_inner()
    }
}

impl Problem for Day5 {
    fn prob1(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob1_inner())
    }

    fn prob2(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob2_inner())
    }
}

#[distributed_slice(PROBLEMS)]
fn register_day(p: &mut HashMap<String, fn() -> Box<dyn Problem>>) {
    p.insert("day5".to_owned(), || Box::new(Day5::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULE_DATA: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

    const TEST_UPDATE_DATA: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_basic_graph() {
        let mut g = Graph::default();

        g.add_node(47);
        g.add_node(53);
        g.add_edge_by_value(47, 53);

        assert_eq!(g.nodes[0].value, 47);
        assert_eq!(g.nodes[1].value, 53);

        assert_eq!(g.index.get(&(47 as usize)), Some(&(0 as usize)));
        assert_eq!(g.index.get(&(53 as usize)), Some(&(1 as usize)));
        assert_eq!(g.index.get(&(77 as usize)), None);

        assert!(g.edge_set.contains(&(47, 53)));

        // This is by index, so node at index 0 is connected to node 1
        assert_eq!(g.successors(0).collect::<Vec<_>>(), vec![1]);
        assert_eq!(g.successors(1).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_parse_rules() {
        let g = parse_rules(TEST_RULE_DATA);

        let s = HashSet::from_iter(g.successors(0));

        let mut expected = HashSet::with_capacity(4);

        expected.insert(*g.index.get(&(53 as usize)).unwrap());
        expected.insert(*g.index.get(&(13 as usize)).unwrap());
        expected.insert(*g.index.get(&(61 as usize)).unwrap());
        expected.insert(*g.index.get(&(29 as usize)).unwrap());

        assert_eq!(s, expected);
    }

    #[test]
    fn test_cycle_detection() {
        let mut g = Graph::default();

        g.add_node(47);
        g.add_node(53);
        g.add_edge_by_value(47, 53);

        assert!(!contains_cycle(&g, &HashSet::from([0, 1])));

        g.add_edge_by_value(53, 47);
        assert!(contains_cycle(&g, &HashSet::from([0, 1])));

        assert!(!contains_cycle(&g, &HashSet::from([0])));
    }

    #[test]
    fn test_problem1_example_only_cycle() {
        let g = parse_rules(TEST_RULE_DATA);

        let data = parse_updates(TEST_UPDATE_DATA);

        let res: Vec<_> = data
            .iter()
            .map(|values| extend_graph_with_values(&g, values))
            .map(|(g, values)| {
                let filter: HashSet<NodeIndex> =
                    HashSet::from_iter(values.iter().map(|v| *g.index.get(&v).unwrap()));
                !contains_cycle(&g, &filter)
            })
            .collect();
        assert_eq!(res, vec![true, true, true, false, false, false]);
    }

    #[test]
    fn test_problem1_whole() {
        let data = TEST_RULE_DATA.to_owned() + "\n\n" + TEST_UPDATE_DATA;
        let day5 = Day5Part1::with_data(&data);
        assert_eq!(day5.prob1_inner().1, 143);
    }

    #[test]
    fn test_topological_sort() {
        let g = parse_rules(TEST_RULE_DATA);
        let res = topological_sort(&g, &vec![75, 97, 47, 61, 53]);
        assert_eq!(res, vec![97, 75, 47, 61, 53]);
        let res = topological_sort(&g, &vec![61, 13, 29]);
        assert_eq!(res, vec![61, 29, 13]);
        let res = topological_sort(&g, &vec![97, 75, 47, 29, 13]);
        assert_eq!(res, vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_problem2_whole() {
        let data = TEST_RULE_DATA.to_owned() + "\n\n" + TEST_UPDATE_DATA;
        let day5 = Day5Part1::with_data(&data).prob1_inner().0;
        assert_eq!(day5.prob2_inner(), 123);
    }

    #[test]
    fn test_actual_problem() {
        let day5 = Day5Part1::new();
        let (day5part2, res) = day5.prob1_inner();
        assert_eq!(res, 4959);
        assert_eq!(day5part2.prob2_inner(), 4655);
    }
}
