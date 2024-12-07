fn main() {
    // Possible operators
    let operators = ['+', '*'];

    // Read the lines of input
    // rust
    let path = "inputs/day7.txt";
    let lines = std::fs::read_to_string(path).unwrap();

    for line in lines.lines() {
        let mut total = 0;

        // Split by space
        let all_numbers: Vec<&str> = line.split_whitespace().collect();

        // The total is the first one, with the colon removed
        let mut target_str = all_numbers[0].chars();
        assert!(target_str.next_back().unwrap() == ':');
        let target_num = target_str.as_str().parse::<i32>().unwrap();

        // Make a vector of numbers
        let numbers = all_numbers[1..].iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // Find how many operators we need
        let n_operators = numbers.len() - 1;

        // Make every possible permutation of operators
        let operator_lists = Vec<Vec<char>>::new();

        // For each permutation, calculate the total
        // Check if this is correct
        // If it is, incrememt the counter
    }
}
