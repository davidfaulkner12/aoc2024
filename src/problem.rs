use std::{collections::HashMap, fmt::Display};

use linkme::distributed_slice;

#[distributed_slice]
pub static PROBLEMS: [fn(&mut HashMap<String, fn() -> Box<dyn Problem>>)];

pub trait Problem {
    fn prob1(&mut self) -> Box<dyn Display>;
    fn prob2(&mut self) -> Box<dyn Display>;
}
