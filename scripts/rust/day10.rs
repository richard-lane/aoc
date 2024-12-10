use std::collections::{HashMap, HashSet};

fn main() {
    let path = "inputs/day10.txt";
    let lines = std::fs::read_to_string(path).unwrap();

    let grid: Vec<Vec<u8>> = lines
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut graph = HashMap::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            let current_val = grid[i][j];
            let mut neighbors = Vec::new();
            if i > 0 && grid[i - 1][j] == current_val + 1 {
                neighbors.push((i - 1, j));
            }
            if i < rows - 1 && grid[i + 1][j] == current_val + 1 {
                neighbors.push((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] == current_val + 1 {
                neighbors.push((i, j - 1));
            }
            if j < cols - 1 && grid[i][j + 1] == current_val + 1 {
                neighbors.push((i, j + 1));
            }
            graph.insert((i, j), neighbors);
        }
    }

    let mut total_score = 0;
    let mut total_rating = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                total_score += count_score(&grid, &graph, (i, j));
                total_rating += rating(&grid, &graph, (i, j));
            }
        }
    }

    println!("Total paths from 0 to 9: {}", total_score);
    println!("Total paths from 0 to 9 with rating: {}", total_rating);
}

fn count_score(
    grid: &Vec<Vec<u8>>,
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
) -> u32 {
    let mut stack = Vec::new();
    stack.push(start);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut score = 0;

    while let Some(current) = stack.pop() {
        if grid[current.0][current.1] == 9 {
            score += 1;
            continue;
        }

        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    stack.push(neighbor);
                }
            }
        }
    }

    score
}

fn rating(
    grid: &Vec<Vec<u8>>,
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
) -> u32 {
    let mut stack = Vec::new();
    stack.push((start, vec![start]));

    let mut paths = 0;

    while let Some((current, path)) = stack.pop() {
        if grid[current.0][current.1] == 9 {
            paths += 1;
            continue;
        }

        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    stack.push((neighbor, new_path));
                }
            }
        }
    }

    paths
}
