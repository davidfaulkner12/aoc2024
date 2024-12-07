use std::iter;

use enum_iterator::Sequence;
use itertools::Itertools;

fn product_with_repeat<T: Clone>(
    it: impl Iterator<Item = T> + Clone,
    cnt: usize,
) -> impl Iterator<Item = Vec<T>> {
    (0..cnt)
        .map(|_| it.clone())
        .into_iter()
        .multi_cartesian_product()
}

fn parse(data: &str) -> Vec<(i64, Vec<i64>)> {
    data.lines()
        .map(|line| {
            let mut split = line.split(": ");
            let res = str::parse(split.next().unwrap()).unwrap();
            let rest = split.next().unwrap();

            (
                res,
                rest.split(" ")
                    .into_iter()
                    .map(|s| str::parse(s).unwrap())
                    .collect(),
            )
        })
        .collect()
}

trait BinOp {
    fn eval(&self, l: i64, r: i64) -> i64;
}

#[derive(Clone, Sequence)]
enum Op {
    Plus,
    Times,
}

impl BinOp for Op {
    fn eval(&self, l: i64, r: i64) -> i64 {
        match self {
            Op::Plus => l + r,
            Op::Times => l * r,
        }
    }
}

fn eval<T: BinOp>(mut ops: Vec<T>, mut operands: Vec<i64>) -> i64 {
    let mut res: i64 = 0;
    while let (Some(op), Some(l), Some(r)) = (ops.pop(), operands.pop(), operands.pop()) {
        let op_res = op.eval(l, r);
        res = op_res;
        operands.push(op_res);
    }
    res
}

#[derive(Clone, Sequence)]
enum ExtendedOp {
    Plus,
    Times,
    Concat,
}

impl BinOp for ExtendedOp {
    fn eval(&self, l: i64, r: i64) -> i64 {
        match self {
            ExtendedOp::Plus => l + r,
            ExtendedOp::Times => l * r,
            ExtendedOp::Concat => str::parse(&(l.to_string() + &r.to_string())).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, convert::identity, fs};

    use super::*;

    const TEST_DATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_product_with_repeat() {
        let res: Vec<Vec<i64>> = product_with_repeat(0..=1, 3).collect();
        assert_eq!(
            res,
            vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 1],
            ]
        );
    }

    #[test]
    fn test_parse() {
        let res = parse(TEST_DATA);
        assert_eq!(res[0], (190, vec![10, 19]));
    }

    #[test]
    fn test_prob1() {
        let data = fs::read_to_string("data/day7.txt").unwrap();
        //let data = TEST_DATA;
        let data = parse(&data);

        assert_eq!(
            //3749 as i64,
            12940396350192 as i64,
            data.iter()
                .filter_map(|(expected, operands)| {
                    let mut stack = operands.clone();
                    stack.reverse();

                    let ops = product_with_repeat(enum_iterator::all::<Op>(), stack.len() - 1);
                    if ops
                        .map(|op| eval(op, stack.clone()))
                        .any(|n| n == *expected)
                    {
                        Some(expected)
                    } else {
                        None
                    }
                })
                .sum()
        );

        assert_eq!(
            //11387 as i64,
            106016735664498 as i64,
            data.iter()
                .filter_map(|(expected, operands)| {
                    let mut stack = operands.clone();
                    stack.reverse();

                    let ops =
                        product_with_repeat(enum_iterator::all::<ExtendedOp>(), stack.len() - 1);
                    if ops
                        .map(|op| eval(op, stack.clone()))
                        .any(|n| n == *expected)
                    {
                        Some(expected)
                    } else {
                        None
                    }
                })
                .sum()
        );
    }
}
