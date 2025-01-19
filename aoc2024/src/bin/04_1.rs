fn main() {
    let result = solve(include_str!("../../input/04.txt"));
    println!("{}", result)
}

fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let target = vec!['X', 'M', 'A', 'S'];
    let target_rev = vec!['S', 'A', 'M', 'X'];
    let height = grid.len();
    let width = grid[0].len();

    let mut result = 0;

    // Check horizontally
    for row in 0..height {
        for col in 0..=width - 4 {
            if (0..4).all(|i| grid[row][col + i] == target[i]) {
                result += 1;
            }
            if (0..4).all(|i| grid[row][col + i] == target_rev[i]) {
                result += 1;
            }
        }
    }

    // Check vertically
    for row in 0..=height - 4 {
        for col in 0..width {
            if (0..4).all(|i| grid[row + i][col] == target[i]) {
                result += 1;
            }
            if (0..4).all(|i| grid[row + i][col] == target_rev[i]) {
                result += 1;
            }
        }
    }

    // Check diagonally down-right
    for row in 0..=height - 4 {
        for col in 0..=width - 4 {
            if (0..4).all(|i| grid[row + i][col + i] == target[i]) {
                result += 1;
            }
            if (0..4).all(|i| grid[row + i][col + i] == target_rev[i]) {
                result += 1;
            }
        }
    }

    // Check diagonally up-right
    for row in 3..height {
        for col in 0..=width - 4 {
            if (0..4).all(|i| grid[row - i][col + i] == target[i]) {
                result += 1;
            }
            if (0..4).all(|i| grid[row - i][col + i] == target_rev[i]) {
                result += 1;
            }
        }
    }

    return result;
}
