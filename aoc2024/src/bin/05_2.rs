use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

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
            let mut pages = line.split(",").collect::<Vec<_>>();
            if pages.is_sorted_by(|a, b| pages_after.get(a).unwrap().contains(b)) {
                return 0;
            }

            pages.sort_by(|a, b| {
                if pages_after.get(a).unwrap().contains(b) {
                    Ordering::Less
                } else if pages_after.get(b).unwrap().contains(a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            pages[pages.len() / 2].parse::<i32>().unwrap()
        })
        .sum()
}
