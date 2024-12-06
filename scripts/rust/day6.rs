fn main() {
    let path = "inputs/day6.txt";
    let input = read_input(path);

    // Find the co-ordinate where the guard starts
    // Find the direction she is moving in
    // Initialise an array storing if the guard has visited a co-ordinate
    // Iterate
    // While true
    // check if the next step is an obstacle
    // if it is, turn right
    // Otherwise, update position
    // If we're outside the grid, break
    // Update the visited array

    // Count the number of visited co-ordinates
}

// Read the text file into a vector of vector of chars
fn read_input(path: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
