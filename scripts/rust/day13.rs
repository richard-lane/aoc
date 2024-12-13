use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let path = "inputs/day13.txt";
    let mut input = String::new();
    let _ = File::open(path)?.read_to_string(&mut input);

    for machine in input.split("\n\n").collect::<Vec<&str>>() {
        let (matrix, prize) = parse(machine);

        println!("Matrix: {:?}", matrix);
        println!("Prize: {:?}", prize);
    }

    Ok(())
}

// Helper function to parse the input into a matrix and vector
fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<i32>) {
    // Init an empty 2x2 matrix
    let mut matrix = vec![vec![0; 2]; 2];
    let mut prize = vec![0; 2];

    let split_input = input.split("\n").collect::<Vec<&str>>();
    assert_eq!(split_input.len(), 3);

    // First line is A
    for i in 0..2 {
        let x = parse_line(split_input[i]);
        matrix[i][0] = x.0;
        matrix[i][1] = x.1;
    }

    // Third line is 3
    let x = parse_line(split_input[2]);
    prize[0] = x.0;
    prize[1] = x.1;

    (matrix, prize)
}

// Get the button coordinates
fn parse_line(input: &str) -> (i32, i32) {
    let parts: Vec<&str> = input.split(", ").collect();
    let x_part = parts[0].split(": ").collect::<Vec<&str>>()[1];
    let y_part = parts[1];
    let x_value: i32 = x_part[2..].parse().unwrap();
    let y_value: i32 = y_part[2..].parse().unwrap();
    (x_value, y_value)
}
