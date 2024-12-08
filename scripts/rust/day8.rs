use std::fs;
use std::collections::HashMap;

fn main () {
    // Parse the input map
    let input = parse_input("inputs/day8.txt");

    // Find antenna positions
    let mut freq_groups: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            if input[r][c] != '.' {
                freq_groups
                    .entry(input[r][c])
                    .or_insert_with(Vec::new)
                    .push((r, c));
            }
        }
    }
    println!("{:?}", freq_groups);
    
    // Find pairs with matching frequencies
    // Find antinode locations by taking the distance between them and adding the distance to the first antinode
    // Handle overlaps
    // Count unique locations
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input)
        .expect("Error reading file")
        .lines()
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}