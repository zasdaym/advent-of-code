fn main() {
    let result = include_str!("../../input/02.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| {
            let size = report.len();
            if is_safe_report(report, None) {
                return true;
            }
            for i in 0..size {
                if is_safe_report(report, Some(i)) {
                    return true;
                }
            }
            return false;
        })
        .count();

    println!("{}", result);
}

fn is_safe_report(report: &[i32], skip: Option<usize>) -> bool {
    let (mut back, mut front) = match skip {
        Some(0) => (1, 2),
        Some(1) => (0, 2),
        _ => (0, 1),
    };

    let is_increasing = report[front] > report[back];

    let skip = match skip {
        Some(x) => x,
        None => report.len() + 1,
    };

    while front < report.len() {
        if front == skip {
            front += 1;
            continue;
        }

        let diff = report[front] - report[back];
        if diff == 0 || diff.abs() > 3 || (is_increasing != (diff > 0)) {
            return false;
        }

        back = front;
        front += 1;
    }

    true
}
