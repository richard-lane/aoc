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
                let (area, perimeter) = dfs(i as i32, j as i32, &grid, &mut visited, grid[i][j]);
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
    current_char: char,
) -> (i32, i32) {
    // Check if we are out of bounds
    if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
        // Out of bounds on a corner contributes 2 to the perimeter
        if (i < 0 && j >= grid[0].len() as i32)
            || (i < 0 && j < 0)
            || (i >= grid.len() as i32 && j < 0)
            || (i >= grid.len() as i32 && j >= grid[0].len() as i32)
        {
            return (0, 2);
        } else {
            return (0, 1);
        }
    }

    // Check if we have already visited this cell
    if visited[i as usize][j as usize] {
        return (0, 0); // No area or perimeter contribution for revisited cells
    }

    // Check if we have reached a new letter
    if grid[i as usize][j as usize] != current_char {
        return (0, 1); // Border with a different region contributes 1 to the perimeter
    }

    // Mark this cell as visited
    visited[i as usize][j as usize] = true;

    // Initialize area and perimeter for this cell
    let mut area = 1;
    let mut perimeter = 0;

    // Recursively visit the neighbors and accumulate area and perimeter
    let neighbors = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
    for (ni, nj) in neighbors {
        let (a, p) = dfs(ni, nj, grid, visited, current_char);
        area += a;
        perimeter += p;
    }

    return (area, perimeter);
}
