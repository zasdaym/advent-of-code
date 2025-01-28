use std::collections::HashSet;

fn main() {
    let result = solve(include_str!("../../input/06.txt"));
    println!("{}", result)
}

fn solve(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut guard = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '^' {
                guard = (i as i32, j as i32);
            }
        }
    }

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;
    let mut stop = false;
    let mut visited = HashSet::new();

    while !stop {
        visited.insert(guard);

        let (delta_row, delta_col) = directions[direction % 4];
        let next_row = guard.0 + delta_row;
        let next_col = guard.1 + delta_col;

        match check_cell(&grid, next_row, next_col) {
            CheckCellResult::Invalid => stop = true,
            CheckCellResult::Obstacle => direction += 1,
            CheckCellResult::Valid => guard = (next_row, next_col),
        }
    }

    visited.len() as i32
}

enum CheckCellResult {
    Invalid,
    Obstacle,
    Valid,
}

fn check_cell(grid: &Vec<Vec<char>>, row: i32, col: i32) -> CheckCellResult {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    if row < 0 || row >= rows || col < 0 || col >= cols {
        return CheckCellResult::Invalid;
    }

    if grid[row as usize][col as usize] == '#' {
        return CheckCellResult::Obstacle;
    }

    CheckCellResult::Valid
}
