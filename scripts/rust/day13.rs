use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let path = "inputs/day13.txt";
    let mut input = String::new();
    let _ = File::open(path)?.read_to_string(&mut input);

    let mut cost = 0;
    for machine in input.split("\n\n").collect::<Vec<&str>>() {
        let (matrix, prize) = parse(machine);

        // Check if the matrix is invertible
        let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
        if det == 0 {
            println!("The matrix is not invertible");
            continue;
        }

        // If it is, solve how many presses we need of each button
        let inv_det = 1.0 / det as f64;
        let inverse_matrix = vec![
            vec![
                matrix[1][1] as f64 * inv_det,
                -matrix[0][1] as f64 * inv_det,
            ],
            vec![
                -matrix[1][0] as f64 * inv_det,
                matrix[0][0] as f64 * inv_det,
            ],
        ];

        let result = vec![
            inverse_matrix[0][0] * prize[0] as f64 + inverse_matrix[0][1] * prize[1] as f64,
            inverse_matrix[1][0] * prize[0] as f64 + inverse_matrix[1][1] * prize[1] as f64,
        ];

        // Check if the result is a whole number
        if result.iter().all(|&x| (x - x.round()).abs() < 1e-5) {
            println!("possible: {:?}", result);
            cost += (3 * result[0].round() as i64) + (result[1].round() as i64);
        } else {
            println!("Result: {:?}", result);
        }
    }

    println!("Cost: {}", cost);
    Ok(())
}

// Helper function to parse the input into a matrix and vector
fn parse(input: &str) -> (Vec<Vec<i64>>, Vec<i64>) {
    // Init an empty 2x2 matrix
    let mut matrix = vec![vec![0; 2]; 2];
    let mut prize = vec![0; 2];

    let split_input = input.split("\n").collect::<Vec<&str>>();
    assert_eq!(split_input.len(), 3);

    // First line is A
    for i in 0..2 {
        let x = parse_line(split_input[i]);
        matrix[0][i] = x.0;
        matrix[1][i] = x.1;
    }

    // Third line is 3
    let x = parse_line(split_input[2]);
    prize[0] = x.0 + 10000000000000;
    prize[1] = x.1 + 10000000000000;

    (matrix, prize)
}

// Get the button coordinates
fn parse_line(input: &str) -> (i64, i64) {
    let parts: Vec<&str> = input.split(", ").collect();
    let x_part = parts[0].split(": ").collect::<Vec<&str>>()[1];
    let y_part = parts[1];
    let x_value: i64 = x_part[2..].parse().unwrap();
    let y_value: i64 = y_part[2..].parse().unwrap();
    (x_value, y_value)
}
