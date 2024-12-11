fn main() {
    let input = std::fs::read_to_string("inputs/day11.txt")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut stones = input;
    for i in 0..25 {
        stones = blink(&stones);
        println!("After {} blinks, there are {} stones", i + 1, stones.len());
    }
}

fn blink(stones: &Vec<i128>) -> Vec<i128> {
    let mut new_stones = Vec::new();
    for stone in stones {
        let val = *stone;
        if val == 0 {
            new_stones.push(1);
        // Check if the stone has an even number of digits
        } else if val.to_string().len() % 2 == 0 {
            // Split the digits in half, create two new stones
            let half = val.to_string().len() / 2;
            let val_str = val.to_string();
            let (left, right) = val_str.split_at(half);
            new_stones.push(left.parse::<i128>().unwrap());
            new_stones.push(right.parse::<i128>().unwrap());
        } else {
            new_stones.push(val * 2024);
        }
    }

    new_stones
}
