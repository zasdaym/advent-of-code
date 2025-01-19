fn main() {
    let input = include_str!("../../input/04.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        println!("0");
        return;
    }

    let height = grid.len();
    let width = grid[0].len();

    if height < 3 || width < 3 {
        println!("0");
        return;
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

    println!("{}", result);
}
