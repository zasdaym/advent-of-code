use regex::Regex;

pub fn main() {
    let pattern = Regex::new(r"don't|do|mul\((\d+),(\d+)\)").unwrap();
    let mut finalizer = 1;
    let result = include_str!("../../input/03.txt")
        .lines()
        .fold(0, |acc, line| {
            acc + pattern
                .captures_iter(line)
                .map(|captures| match &captures[0] {
                    "don't" => {
                        finalizer = 0;
                        0
                    }
                    "do" => {
                        finalizer = 1;
                        0
                    }
                    _ => {
                        let a = captures[1].parse::<i32>().unwrap();
                        let b = captures[2].parse::<i32>().unwrap();
                        a * b * finalizer
                    }
                })
                .sum::<i32>()
        });

    println!("{}", result);
}
