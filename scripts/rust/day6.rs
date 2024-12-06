use std::collections::HashMap;

fn main() {
    let path = "inputs/day6.txt";
    let input = read_input(path);

    // Find the co-ordinate where the guard starts
    let my_dict: HashMap<char, (i32, i32)> =
        [('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0)), ('v', (1, 0))]
            .iter()
            .cloned()
            .collect();

    // Find the direction she is moving in and her location
    let mut found = false;
    let mut loc = (0, 0);
    let mut direction = (0, 0);
    for (i, row) in input.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if found {
                break;
            };
            if my_dict.contains_key(col) {
                if let Some(&dir) = my_dict.get(col) {
                    loc = (i as usize, j as usize);
                    direction = dir;
                    found = true;
                }
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

    println!("{:?}, {:?}", loc, direction);

    // Initialise an array storing if the guard has visited a co-ordinate
    // Iterate
    // While true
    // check if the next step is an obstacle
    // if it is, turn right
    // Otherwise, update position
    // If we're outside the grid, break
    // Update the visited array

    // Count the number of visited co-ordinates
}

// Read the text file into a vector of vector of chars
fn read_input(path: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
