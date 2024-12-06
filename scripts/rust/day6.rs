use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum MyError {
    StuckInLoop,
    GuardNotFound,
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::StuckInLoop => write!(f, "Stuck in a loop"),
            MyError::GuardNotFound => write!(f, "Guard not found"),
        }
    }
}

// Implement the std::error::Error trait for MyError
impl std::error::Error for MyError {}

fn main() {
    let path = "inputs/day6.txt";
    let input = read_input(path);

    // Find the co-ordinate where the guard starts
    let dirn_map: HashMap<char, (i32, i32)> =
        [('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0)), ('v', (1, 0))]
            .iter()
            .cloned()
            .collect();

    let (loc, direction) = initial_conditions(&input, &dirn_map).unwrap();

    // Part 1 - just count the number of visits
    let visited = find_visited(&input, loc, direction).unwrap();
    let count = visited.iter().flatten().filter(|&&x| x == 'X').count();
    println!("Number of houses visited: {}", count);

    // Part 2 - try adding an obstacle to each location, and see if we get stuck
    let mut n_loops = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '#' || dirn_map.contains_key(&input[i][j]) {
                continue;
            }
            let mut new_input = input.clone();
            new_input[i][j] = '#';

            match find_visited(&new_input, loc, direction) {
                Ok(_) => (),
                Err(MyError::StuckInLoop) => {
                    n_loops += 1;
                }
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }
    }
    println!("Number of obstacles causing a loop: {}", n_loops);
}

// Find the locations visited given the input
fn find_visited(
    input: &Vec<Vec<char>>,
    mut loc: (i32, i32),
    mut direction: (i32, i32),
) -> Result<Vec<Vec<char>>, MyError> {
    // Initialise an array storing if the guard has visited a co-ordinate
    let mut visited = vec![vec!['.'; input[0].len()]; input.len()];

    let mut n_visited = 0;
    loop {
        visited[loc.0 as usize][loc.1 as usize] = 'X';
        n_visited += 1;
        // If this is more than the total size of the grid, raise; we've clearly got stuck in a loop
        if n_visited > input.len() * input[0].len() {
            return Err(MyError::StuckInLoop);
        }

        let next_loc = ((loc.0 + direction.0), (loc.1 + direction.1));

        // Find out if the next step is outside of the grid
        if next_loc.0 < 0
            || next_loc.0 >= input.len() as i32
            || next_loc.1 < 0
            || next_loc.1 >= input[0].len() as i32
        {
            break;
        }

        // Find out if the next co-ordinate is a wall
        if input[next_loc.0 as usize][next_loc.1 as usize] == '#' {
            direction = match direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };
            loc = ((loc.0 + direction.0), (loc.1 + direction.1));
        } else {
            loc = next_loc;
        };
    }

    return Ok(visited);
}

// Find the initial location and direction
fn initial_conditions(
    input: &Vec<Vec<char>>,
    dirn_map: &HashMap<char, (i32, i32)>,
) -> Result<((i32, i32), (i32, i32)), MyError> {
    // Find the direction she is moving in and her location
    let mut found = false;
    let mut loc = (0, 0);
    let mut direction = (0, 0);

    for (i, row) in input.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if found {
                break;
            };
            if dirn_map.contains_key(col) {
                loc = (i as i32, j as i32);
                direction = dirn_map.get(col).unwrap().clone();
                found = true;
            }
        }
        if found {
            break;
        };
    }
    // If we didn't break, raise an error
    if !found {
        return Err(MyError::GuardNotFound);
    }

    Ok((loc, direction))
}

// Read the text file into a vector of vector of chars
fn read_input(path: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
