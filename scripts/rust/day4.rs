use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "inputs/day4.txt";

    // Read the file into a grid
    let contents = read_file(path)?;

    let directions = vec![
        (0, 1),   // right
        (1, 0),   // down
        (0, -1),  // left
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];
    let mut occurrences = 0;

    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
            for &(dx, dy) in &directions {
                if check_word(&contents, "XMAS", i, j, dx, dy) {
                    occurrences += 1;
                }
            }
        }
    }

    println!("Occurrences: {}", occurrences);

    Ok(())
}

// Helper function to check if a word is present in a grid
fn check_word(
    grid: &Vec<String>,
    target: &str,
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
) -> bool {
    let target_len = target.len();
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    for k in 0..target_len {
        let x = start_x as isize + k as isize * dx;
        let y = start_y as isize + k as isize * dy;

        // Check if we are out of bounds
        if x < 0 || x >= grid_height as isize || y < 0 || y >= grid_width as isize {
            return false;
        }

        if grid[x as usize].chars().nth(y as usize).unwrap() != target.chars().nth(k).unwrap() {
            return false;
        }
    }

    true
}

// Helper function to read a file into a vector of vectors
fn read_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut contents = Vec::new();

    for line in reader.lines() {
        let line = line?;
        contents.push(line);
    }

    Ok(contents)
}
