use problem::Problem;

mod day1;
mod day2;
mod day3;
mod poc;
mod problem;

fn get_problem(s: &str) -> Box<dyn Problem> {
    Box::new(day1::Day1::new())
}

fn main() {
    let mut problem = get_problem("day1");
    let prob1 = problem.prob1();
    let prob2 = problem.prob2();

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2);
}
