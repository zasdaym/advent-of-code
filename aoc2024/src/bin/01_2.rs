use std::{collections::HashMap, fs::read_to_string};

pub fn main() {
    let content = read_to_string("input/01.txt").unwrap();

    let (left, right): (Vec<i32>, HashMap<i32, i32>) = content
        .lines()
        .map(|line| {
            let (num1, num2) = line.split_once("   ").unwrap();
            (num1.parse().unwrap(), num2.parse().unwrap())
        })
        .fold(
            (vec![], HashMap::new()),
            |(mut nums, mut counts), (n1, n2)| {
                nums.push(n1);
                *counts.entry(n2).or_insert(0) += 1;
                (nums, counts)
            },
        );

    let result: i32 = left.iter().map(|n| n * right.get(n).unwrap_or(&0)).sum();

    println!("{}", result);
}
