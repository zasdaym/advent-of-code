fn main() {
    let result = solve(include_str!("../../input/04.txt"));
    println!("{}", result);
}

fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        return 0;
    }

    let height = grid.len();
    let width = grid[0].len();

    if height < 3 || width < 3 {
        return 0;
    }

    let mut result = 0;

    for row in 0..=height - 3 {
        for col in 0..=width - 3 {
            if grid[row + 1][col + 1] != 'A' {
                continue;
            }

            let top_left = grid[row][col];
            let top_right = grid[row][col + 2];
            let bottom_left = grid[row + 2][col];
            let bottom_right = grid[row + 2][col + 2];

            if !matches!(top_left, 'S' | 'M')
                || !matches!(top_right, 'S' | 'M')
                || !matches!(bottom_left, 'S' | 'M')
                || !matches!(bottom_right, 'S' | 'M')
            {
                continue;
            }

            if top_left != bottom_right && bottom_left != top_right {
                result += 1;
            }
        }
    }

    return result;
}
