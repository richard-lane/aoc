//! ```cargo
//! [dependencies]
//! itertools = "0.10"
//! ```

use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    // Possible operators
    let operators = ['+', '*'];

    // Read the lines of input
    // rust
    let path = "inputs/day7.txt";
    let lines = std::fs::read_to_string(path).unwrap();

    let mut n_correct_lines = 0;

    for line in lines.lines() {
        // Split by space
        let all_numbers: Vec<&str> = line.split_whitespace().collect();

        // The total is the first one, with the colon removed
        let mut target_str = all_numbers[0].chars();
        assert!(target_str.next_back().unwrap() == ':');
        let target_num = target_str.as_str().parse::<i32>().unwrap();

        // Make a vector of numbers
        let numbers = all_numbers[1..]
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Find how many operators we need
        let n_operators = numbers.len() - 1;

        // Make every possible permutation of operators
        let operator_combinations: HashSet<Vec<char>> = operators
            .iter()
            .cloned()
            .combinations_with_replacement(n_operators)
            .flat_map(|combo| combo.into_iter().permutations(n_operators))
            .map(|perm| perm.into_iter().collect::<Vec<char>>())
            .collect();

        // For each permutation, calculate the total
        for operator_combo in operator_combinations {
            let mut result = numbers[0];
            for (i, &op) in operator_combo.iter().enumerate() {
                match op {
                    '+' => result += numbers[i + 1],
                    '*' => result *= numbers[i + 1],
                    _ => panic!("Unexpected operator"),
                }
            }
            if result == target_num {
                n_correct_lines += 1;
                break;
            }
        }
    }

    println!("{}", n_correct_lines);
}
