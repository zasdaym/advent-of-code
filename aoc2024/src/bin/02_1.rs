fn main() {
    let result = include_str!("../../input/02.txt")
        .lines()
        .filter(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            if report.len() < 2 {
                return false;
            }

            let is_increasing = report[1] > report[0];

            report.windows(2).all(|pair| {
                let diff = pair[1] - pair[0];

                diff != 0
                    && diff.abs() <= 3
                    && ((is_increasing && diff > 0) || (!is_increasing && diff < 0))
            })
        })
        .count();

    println!("{}", result);
}
