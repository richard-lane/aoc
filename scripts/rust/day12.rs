fn main() {
    // Read in the input as a 2d vector of chars
    let path = "inputs/day12.txt";
    let content = std::fs::read_to_string(path).unwrap();
    let grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    // Perform a depth first search to find the number of connected components
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut cost = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                // We have found a new connected component
                let (area, perimeter) =
                    dfs(i as i32, j as i32, &grid, &mut visited, 0, 0, grid[i][j]);
                println!("Area: {}, Perimeter: {}", area, perimeter);
                cost += area * perimeter;
            }
        }
    }

    println!("{}", cost);
}

fn dfs(
    i: i32,
    j: i32,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    area: i32,
    perimeter: i32,
    current_char: char,
) -> (i32, i32) {
    // Check if we are out of bounds
    if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
        return (area, perimeter);
    }

    // Check if we have already visited this cell
    if visited[i as usize][j as usize] {
        return (area, perimeter);
    }

    // Check if we have reached a new letter
    if grid[i as usize][j as usize] != current_char {
        return (area, perimeter + 1);
    }

    // Mark the cell as visited
    visited[i as usize][j as usize] = true;

    // Recursively visit the neighbors
    let (area, perimeter) = dfs(i + 1, j, grid, visited, area + 1, perimeter, current_char);
    let (area, perimeter) = dfs(i - 1, j, grid, visited, area + 1, perimeter, current_char);
    let (area, perimeter) = dfs(i, j + 1, grid, visited, area + 1, perimeter, current_char);
    let (area, perimeter) = dfs(i, j - 1, grid, visited, area + 1, perimeter, current_char);

    return (area, perimeter);
}
