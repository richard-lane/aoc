use std::collections::{HashMap, HashSet};

fn main() {
    let path = "inputs/day5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    for l in rules.lines() {
        let (x, y) = l.split_once('|').unwrap();
        orderings
            .entry(y.parse().unwrap())
            .or_default()
            .insert(x.parse().unwrap());
    }

    let pages = updates.lines().map(|l| {
        l.split(',')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let (mut p1, mut p2) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| orderings[b].contains(a)) {
            p1 += p[p.len() / 2];
        } else {
            p.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
            p2 += p[p.len() / 2];
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
