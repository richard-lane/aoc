use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day3.txt";

    // Read the file into a string
    let contents = read_file(path)?;

    // Record which indices correspond to the mul( characters
    let mul_indices = find_match(&contents, "mul(");

    // Record which indices correspond to do() and don't()
    let do_indices = find_match(&contents, "do()");
    let dont_indices = find_match(&contents, "don't()");

    // Track if multiplication is enabled
    let mut enabled = true;

    let mut result = 0;
    for index in mul_indices {
        // Find if this index is enabled
        // If this index is > do[0], we are now enabled
        // If this index is > dont[0] we are now disabled

        if enabled {
            result += mul_numbers(&contents, index);
        }
    }
    println!("Result: {}", result);

    Ok(())
}

// Helper function to read a file into one string
fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Find which indices correspond to the mul( characters
fn find_match(s: &str, substr: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, window) in s
        .chars()
        .collect::<Vec<char>>()
        .windows(substr.len())
        .enumerate()
    {
        if window.into_iter().collect::<String>() == substr {
            indices.push(i);
        }
    }
    indices
}

// Starting at a given index, find the numbers that need to be multiplied
// and multiply them
// If the numbers are not valid, return 0
fn mul_numbers(s: &str, index: usize) -> i32 {
    let mut n1 = Vec::new();
    let mut n2 = Vec::new();
    let mut n1_done = false;

    for i in index + 4..index + 12 {
        // Check if it is a digit
        let c = s.chars().nth(i).unwrap();
        if c.is_digit(10) {
            // If we're not done with the first number, add it to n1
            if !n1_done {
                n1.push(s.chars().nth(i).unwrap());
            } else {
                n2.push(s.chars().nth(i).unwrap());
            }
        } else if c == ',' {
            if !n1_done {
                n1_done = true;
            } else {
                // Second comma encountered - invalid mul
                return 0;
            }
        } else if c == ')' {
            // Check if we have valid numbers
            if n1.len() == 0 || n2.len() == 0 {
                return 0;
            }

            // We're done - turn the vectors into numbers and parse them
            let n1 = n1.into_iter().collect::<String>().parse::<i32>().unwrap();
            let n2 = n2.into_iter().collect::<String>().parse::<i32>().unwrap();
            return n1 * n2;
        } else {
            // Invalid character
            return 0;
        }
    }
    // We didn't find a close bracket
    0
}
