use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day5.txt";

    // Read the file and process the inputs
    let (map, vectors) = process_file(path)?;

    // Print the map for verification
    println!("Map:");
    for (key, values) in &map {
        println!("{}: {:?}", key, values);
    }

    // Print the vectors for verification
    println!("\nVectors:");
    for vector in &vectors {
        println!("{:?}", vector);
    }

    Ok(())
}

fn process_file<P>(filename: P) -> io::Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    let mut vectors = Vec::new();
    let mut before_blank_line = true;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            before_blank_line = false;
            continue;
        }

        if before_blank_line {
            let parts: Vec<&str> = line.split('|').collect();
            let left: i32 = parts[0].parse().unwrap();
            let right: i32 = parts[1].parse().unwrap();
            map.entry(left).or_insert_with(Vec::new).push(right);
        } else {
            let vector: Vec<i32> = line.split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            vectors.push(vector);
        }
    }

    Ok((map, vectors))
}