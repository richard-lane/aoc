use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day1.txt";
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                let nums: Vec<&str> = ip.split_whitespace().collect();
                if nums.len() == 2 {
                    if let (Ok(a), Ok(b)) = (nums[0].parse::<i32>(), nums[1].parse::<i32>()) {
                        l1.push(a);
                        l2.push(b);
                    }
                } else {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid input: {:?}", nums)));
                }
            }
        }
    }

    l1.sort();
    l2.sort();

    // Find the difference between each element in the list and sum them
    let mut sum = 0;
    for (a, b) in l1.iter().zip(l2.iter()) {
        sum += (b - a).abs();
    }
    println!("Part1: {:?}", sum);

    // Find the number of occurrences
    let mut part2 = 0;
    for a in l1.iter() {
        part2 += a * l2.iter().filter(|&x| x == a).count() as i32;
    }
    println!("Part2: {:?}", part2);

    Ok(())
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}