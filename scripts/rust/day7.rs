fn main() {
    // Possible operators
    let operators = ['+', '*'];

    // Read the lines of input
    // rust
    let path = "inputs/day7.txt";
    let lines = std::fs::read_to_string(path).unwrap();

    for line in lines.lines() {
        // Split by space
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // The total is the first one, with the colon removed
        let mut target_str = numbers[0].chars();
        assert!(target_str.next_back().unwrap() == ':');
        let target_num = target_str.as_str().parse::<i32>().unwrap();
        println!("{:?}", target_num)
    }
    // Make a vector of numbers
    // Find how many operators we need
    // Make every possible permutation of operators
    // For each permutation, calculate the total
    // Check if this is correct
    // If it is, incrememt the counter
}
