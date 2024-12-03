use std::fmt::Display;

pub trait Problem {
    fn prob1(&mut self) -> Box<dyn Display>;
    fn prob2(&mut self) -> Box<dyn Display>;
}
