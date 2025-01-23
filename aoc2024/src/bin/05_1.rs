use std::collections::{HashMap, HashSet};

fn main() {
    let result = solve(include_str!("../../input/05.txt"));
    println!("{}", result)
}

fn solve(input: &str) -> i32 {
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let mut pages_after = HashMap::new();
    for line in orderings.lines() {
        let (a, b) = line.split_once("|").unwrap();
        pages_after.entry(a).or_insert(HashSet::new()).insert(b);
    }

    updates
        .lines()
        .map(|line| {
            let pages = line.split(",").collect::<Vec<_>>();
            if pages.is_sorted_by(|a, b| pages_after.get(a).unwrap().contains(b)) {
                pages[pages.len() / 2].parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .sum()
}
