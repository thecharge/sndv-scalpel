use std::fmt::Display;

pub struct Worker;

pub fn calculate_total(items: &[i32]) -> i32 {
    let mut sum = 0;
    for item in items {
        sum += item;
    }
    sum
}

impl Worker {
    pub fn run(&self) {}
}
