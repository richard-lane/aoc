use std::collections::HashMap;

fn main() {
    let path = "inputs/day6.txt";
    let input = read_input(path);

    // Find the co-ordinate where the guard starts
    let dirn_map: HashMap<char, (i32, i32)> =
        [('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0)), ('v', (1, 0))]
            .iter()
            .cloned()
            .collect();

    let (mut loc, mut direction) = initial_conditions(&input, &dirn_map);

    // Initialise an array storing if the guard has visited a co-ordinate
    let mut visited = vec![vec!['.'; input[0].len()]; input.len()];

    loop {
        visited[loc.0 as usize][loc.1 as usize] = 'X';

        let next_loc = ((loc.0 + direction.0), (loc.1 + direction.1));

        // Find out if the next step is outside of the grid
        if next_loc.0 < 0
            || next_loc.0 >= input.len() as i32
            || next_loc.1 < 0
            || next_loc.1 >= input[0].len() as i32
        {
            for row in &visited {
                println!("{:?}", row);
            }
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

        // Find out if we have visited this co-ordinate before and are facing the same direction
    }

    // Count the number of visited co-ordinates
    // THis is the number of Xs
    let count = visited.iter().flatten().filter(|&&x| x == 'X').count();
    println!("Number of houses visited: {}", count);
}

// Find the initial location and direction
fn initial_conditions(
    input: &Vec<Vec<char>>,
    dirn_map: &HashMap<char, (i32, i32)>,
) -> ((i32, i32), (i32, i32)) {
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
        panic!("Guard not found");
    }

    (loc, direction)
}

// Read the text file into a vector of vector of chars
fn read_input(path: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
