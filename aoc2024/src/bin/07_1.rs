fn main() {
    let result = solve(include_str!("../../input/07.txt"));
    println!("{}", result)
}

fn solve(input: &str) -> i128 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let target: i128 = left.parse().unwrap();
            let operands: Vec<i128> = right
                .split_whitespace()
                .map(|c| c.parse::<i128>().unwrap())
                .collect();

            // Try all possible combinations of + and * operators
            let mut stack = vec![operands[0]];
            for &num in &operands[1..] {
                let mut new_stack = Vec::new();
                for &curr_sum in &stack {
                    new_stack.push(curr_sum + num);
                    new_stack.push(curr_sum * num);
                }
                stack = new_stack;
            }

            // Check if any combination equals target
            if stack.iter().any(|&result| result == target) {
                target
            } else {
                0
            }
        })
        .sum()
}
