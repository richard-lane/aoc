use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let path = "inputs/day10.txt";
    let lines = std::fs::read_to_string(path).unwrap();

    let grid: Vec<Vec<u8>> = lines
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    println!("{:?}", grid)
}
