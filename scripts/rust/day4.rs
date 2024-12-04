use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day4.txt";

    // Read the file into a grid
    let contents = read_file(path)?;

    println!("{:?}", contents);

    Ok(())
}

// Helper function to read a file into a vector of vectors
fn read_file<P>(filename: P) -> io::Result<Vec<Vec<char>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut contents = Vec::new();

    for line in reader.lines() {
        let line = line?;
        contents.push(line.chars().collect());
    }

    Ok(contents)
}
