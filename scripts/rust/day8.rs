use std::fs;

fn main () {
    // Parse the input map
    let input = parse_input("inputs/day8.txt");

    // Find antenna positions
    // Find pairs with matching frequencies
    // Find antinode locations
    // Handle overlaps
    // Count unique locations
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input).expect("Error reading file")
        .lines()
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}