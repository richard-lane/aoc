fn main() {
    // Read in the input as a 2d vector of chars
    let path = "inputs/day12.txt";
    let content = std::fs::read_to_string(path).unwrap();
    let grid: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    println!("{:?}", grid)
}
