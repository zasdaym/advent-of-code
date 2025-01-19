use regex::Regex;

pub fn main() {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = include_str!("../../input/03.txt")
        .lines()
        .map(|line| {
            pattern
                .captures_iter(line)
                .map(|captures| {
                    let a = captures[1].parse::<i32>().unwrap();
                    let b = captures[2].parse::<i32>().unwrap();
                    a * b
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{}", result);
}
