use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day2.txt";
    let mut report: Vec<Vec<i32>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                let nums: Vec<i32> = ip
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().expect("parse error"))
                    .collect();
                report.push(nums);
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input"));
    }

    let mut n_valid = 0;
    for r in report.iter() {
        if valid_with_dampener(r) {
            n_valid += 1;
        }
    }
    println!("Part1: {:?}", n_valid);

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

// Find if a report is valid
fn is_valid(report: &Vec<i32>) -> bool {
    // A report is invalid if it is not monotic
    // Or if the difference between consecutive elements is greater than 3

    let mut ascending: Option<bool> = None;
    for x in report.windows(2) {
        let diff = x[1] - x[0];
        // Check if we're ascending or descending
        if ascending.is_none() {
            ascending = Some(diff > 0);
        }

        // Check for monotonicity
        if ascending.unwrap() != (diff > 0) {
            return false;
        }

        let diff = (x[1] - x[0]).abs();
        if diff > 3 || diff == 0 {
            return false;
        }
    }

    return true;
}

// Find if a report is valid, with the problem dampener applied
fn valid_with_dampener(report: &Vec<i32>) -> bool {
    // If it is valid, then that's fine
    if is_valid(report) {
        return true;
    }

    // If not, we'll have to remove the elements one at a time
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if is_valid(&new_report) {
            return true;
        }
    }

    return false;
}
