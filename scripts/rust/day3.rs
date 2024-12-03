use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day3.txt";

    // Read the file into a string
    let contents = read_file(path)?;

    // Record which indices correspond to the mul( characters
    let mul_indices = find_muls(&contents);
    println!("{:?}", mul_indices);

    // Find out which ones are valid
    // perform multiplications
    // Add the results

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
fn find_muls(s: &str) -> Vec<usize> {
    let mut mul_indices = Vec::new();
    for (i, substr) in s.chars().collect::<Vec<char>>().windows(4).enumerate() {
        if substr.into_iter().collect::<String>() == "mul(" {
            mul_indices.push(i);
        }
    }
    mul_indices
}
