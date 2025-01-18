use std::fs::read_to_string;

pub fn main() {
    let content = read_to_string("input/01.txt").unwrap();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = content
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .fold((vec![], vec![]), |(mut left, mut right), (l, r)| {
            left.push(l);
            right.push(r);
            (left, right)
        });

    left.sort_unstable();
    right.sort_unstable();

    let result: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{}", result)
}
