use std::fs;

use regex::Regex;

use crate::problem::Problem;

fn get_pairs(s: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(s)
        .map(|caps| {
            let (_, [l, r]) = caps.extract();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

fn get_pairs_stateful(s: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(don't)|(do)").unwrap();
    let (_, acc) = re
        .captures_iter(s)
        .fold((true, Vec::new()), |(enabled, mut acc), caps| {
            if caps.get(4).is_some() {
                (false, acc)
            } else if caps.get(5).is_some() {
                (true, acc)
            } else if !enabled {
                (enabled, acc)
            } else {
                let l = &caps[2];
                let r = &caps[3];
                acc.push((l.parse().unwrap(), r.parse().unwrap()));
                (enabled, acc)
            }
        });

    acc
}

#[derive(Default)]
pub struct Day3 {
    data: String,
}

impl Day3 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day3.txt").unwrap();
        Day3 {
            data: data.to_owned(),
        }
    }
    fn prob1_inner(&mut self) -> usize {
        let pairs = get_pairs(&self.data);
        pairs.iter().map(|(l, r)| l * r).sum()
    }
    fn prob2_inner(&mut self) -> usize {
        let pairs = get_pairs_stateful(&self.data);
        pairs.iter().map(|(l, r)| l * r).sum()
    }
}

impl Problem for Day3 {
    fn prob1(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob1_inner())
    }

    fn prob2(&mut self) -> Box<dyn std::fmt::Display> {
        Box::new(self.prob2_inner())
    }
}

#[cfg(test)]
mod tests {

    use super::{get_pairs, get_pairs_stateful, Day3};

    const TEST_DATA: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_basics_parse() {
        let pairs = get_pairs(TEST_DATA);
        assert_eq!(pairs, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
        let res: usize = pairs.iter().map(|(l, r)| l * r).sum();
        assert_eq!(res, 161);
    }

    const TEST_DATA_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_prob2() {
        let pairs = get_pairs_stateful(TEST_DATA_2);
        assert_eq!(pairs, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn test_prob() {
        let mut day3 = Day3::new();
        assert_eq!(day3.prob1_inner(), 183380722);
        assert_eq!(day3.prob2_inner(), 82733683);
    }

    use chumsky::prelude::*;

    #[derive(Debug, PartialEq)]
    enum Expr {
        Null,
        Num(u64),
        Seq(Vec<Box<Expr>>),
        Mul(u64, u64),
    }

    fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
        let int = text::int(10).map(|s: String| Expr::Num(s.parse().unwrap()));
        let junk = int.not().ignored().map(|_| Expr::Null);

        choice((dont_do(), int, junk)).repeated().map(|v| {
            Expr::Seq(
                v.into_iter()
                    .filter(|e| !matches!(e, Expr::Null))
                    .map(Box::new)
                    .collect(),
            )
        })
    }

    fn mul() -> impl Parser<char, Expr, Error = Simple<char>> {
        let int = text::int(10).map(|s: String| s.parse().unwrap());
        just("mul(")
            .ignore_then(int)
            .then_ignore(just(","))
            .then(int)
            .then_ignore(just(")"))
            .map(|(l, r)| Expr::Mul(l, r))
    }

    fn dont_do() -> impl Parser<char, Expr, Error = Simple<char>> {
        let do_end = just("don't").not().rewind().then(just("do"));
        do_end
            .clone()
            .not()
            .repeated()
            .ignored()
            .map(|_| Expr::Null)
            .delimited_by(just("don't"), do_end)
    }

    #[test]
    fn test_parser() {
        let p = parser();
        assert_eq!(
            p.parse("392"),
            Ok(Expr::Seq(vec![Box::new(Expr::Num(392))]))
        );
        assert_eq!(p.parse("a"), Ok(Expr::Seq(vec![])));
        assert_eq!(
            p.parse("a393ajsdfjkl39320"),
            Ok(Expr::Seq(vec![
                Box::new(Expr::Num(393)),
                Box::new(Expr::Num(39320))
            ]))
        );
    }

    #[test]
    fn test_mul() {
        let p = mul();
        assert_eq!(p.parse("mul(192,39)"), Ok(Expr::Mul(192, 39)))
    }

    #[test]
    fn test_dont_do() {
        let p = dont_do();
        assert_eq!(p.parse("don'tabcdefghdo"), Ok(Expr::Null));
        assert_eq!(p.parse("don'tabcdefghdon'tjaksdfjdo"), Ok(Expr::Null));
    }

    #[test]
    fn test_dont_do_number() {
        let p = parser();
        assert_eq!(
            p.parse("asdf393ajsddon't39203asdf3928123do3921932"),
            Ok(Expr::Seq(vec![
                Box::new(Expr::Num(393)),
                Box::new(Expr::Num(3921932))
            ]))
        );
    }

    #[test]
    fn test_prob2_example_chumsky() {
        let junk = mul().not().ignored().map(|_| Expr::Null);
        let p = choice((dont_do(), mul(), junk)).repeated().map(|v| {
            Expr::Seq(
                v.into_iter()
                    .filter(|e| !matches!(e, Expr::Null))
                    .map(Box::new)
                    .collect(),
            )
        });
        let res = p.parse(TEST_DATA_2);
        assert_eq!(
            res,
            Ok(Expr::Seq(vec![
                Box::new(Expr::Mul(2, 4)),
                Box::new(Expr::Mul(8, 5)),
            ]))
        );
    }
}
