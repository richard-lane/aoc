use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day3.txt";

    // Read the file into a string
    let contents = read_file(path)?;
    println!("{:?}", contents);

    // Record which indices correspond to the mul( characters
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