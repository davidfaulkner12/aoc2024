use std::{collections::HashMap, env::args};

use problem::{Problem, PROBLEMS};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod problem;

fn main() {
    let mut problems: HashMap<String, fn() -> Box<dyn Problem>> = HashMap::new();

    for p in PROBLEMS {
        p(&mut problems);
    }

    let mut problem = problems.get(&args().nth(1).unwrap()).unwrap()();
    let prob1 = problem.prob1();
    let prob2 = problem.prob2();

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2);
}
