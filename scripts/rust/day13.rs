use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let path = "inputs/day13.txt";
    let mut input = String::new();
    let _ = File::open(path)?.read_to_string(&mut input);

    for machine in input.split("\n\n").collect::<Vec<&str>>() {
        println!("{}\n", machine);
    }
    //     let line = line?;
    //     if line.starts_with("Button A:") || line.starts_with("Button B:") {
    //         let parts: Vec<&str> = line.split(", ").collect();
    //         let x_part = parts[0].split(": ").collect::<Vec<&str>>()[1];
    //         let y_part = parts[1];
    //         let x_value: i32 = x_part[2..].parse().unwrap();
    //         let y_value: i32 = y_part[2..].parse().unwrap();
    //         matrix[0].push(x_value);
    //         matrix[1].push(y_value);
    //     } else if line.starts_with("Prize:") {
    //         let parts: Vec<&str> = line.split(", ").collect();
    //         let x_part = parts[0].split("=").collect::<Vec<&str>>()[1];
    //         let y_part = parts[1].split("=").collect::<Vec<&str>>()[1];
    //         let x_value: i32 = x_part.parse().unwrap();
    //         let y_value: i32 = y_part.parse().unwrap();
    //         prize.push(x_value);
    //         prize.push(y_value);
    //     }
    // }

    // println!("Matrix: {:?}", matrix);
    // println!("Prize: {:?}", prize);

    Ok(())
}

// Helper function to parse the input into a matrix and vector
fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<i32>) {
    println!("{:?}", input);

    (Vec::new(), Vec::new())
}
