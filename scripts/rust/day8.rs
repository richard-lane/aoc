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

    // Init an empty grid storing the antinode locations
    let mut antinode_grid = vec![vec!['0'; input[0].len()]; input.len()];

    // Find pairs with matching frequencies
    for positions in freq_groups.values() {
        for pos1 in positions.iter() {
            for pos2 in positions.iter() {
                if pos1 == pos2 {
                    continue;
                }
                // Find the distance between them
                let dx = (pos1.0 - pos2.0) as i32;
                let dy = (pos1.1 - pos2.1) as i32;

                // Possible antinode locations
                let mut x1 = pos1.0 as i32 + dx;
                let mut y1 = pos1.1 as i32 + dy;

                let mut x2 = pos2.0 as i32 - dx;
                let mut y2 = pos2.1 as i32 - dy;

                // Check if they're in bounds
                while x1 >= 0 && x1 < input.len() as i32 && y1 >= 0 && y1 < input[0].len() as i32 {
                    antinode_grid[x1 as usize][y1 as usize] = 'A';

                    x1 += dx;
                    y1 += dy;
                }
                while x2 >= 0 && x2 < input.len() as i32 && y2 >= 0 && y2 < input[0].len() as i32 {
                    antinode_grid[x2 as usize][y2 as usize] = 'A';

                    x2 -= dx;
                    y2 -= dy;
                }
            }
        }
    }
    // Count unique locations
    println!("{}", antinode_grid.iter().flatten().filter(|&c| *c == 'A').count());
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