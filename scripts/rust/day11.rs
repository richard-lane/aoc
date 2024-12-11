use std::collections::HashMap;

fn main() {
    // Store how many of each number we have in a HashMap
    let path = "inputs/day11.txt";
    let content = std::fs::read_to_string(path).unwrap();
    let mut counts = HashMap::new();
    for num in content
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
    {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Apply the rules to the stones
    for i in 0..75 {
        counts = update(&counts);
        println!("After {} iterations: {:?}", i + 1, counts.values().sum::<usize>());
    }
}

fn update(stones: &HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (&k, &v) in stones {
        match k {
            0 => *new_stones.entry(1).or_default() += v,
            _ => {
                let n_digits = k.ilog10() + 1;
                if n_digits % 2 == 0 {
                    *new_stones.entry(k % 10i64.pow(n_digits / 2)).or_default() += v;
                    *new_stones.entry(k / 10i64.pow(n_digits / 2)).or_default() += v;
                } else {
                    *new_stones.entry(k * 2024).or_default() += v;
                }
            }
        }
    }
    new_stones
}
