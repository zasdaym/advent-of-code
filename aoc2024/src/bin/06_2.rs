use std::collections::HashSet;

fn main() {
    let result = solve(include_str!("../../input/06.txt"));
    println!("{}", result)
}

fn solve(input: &str) -> i32 {
    let mut grid = input
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

    let exit_path = find_exit_path(&grid, guard);
    let mut result = 0;

    for (idx, &pos) in exit_path.iter().enumerate() {
        if grid[pos.0 as usize][pos.1 as usize] != '.' {
            continue;
        }
        grid[pos.0 as usize][pos.1 as usize] = '#';
        if check_loop(&grid, guard) {
            result += 1;
        }
        grid[pos.0 as usize][pos.1 as usize] = '.';
    }

    result
}

fn find_exit_path(grid: &Vec<Vec<char>>, guard_start: (i32, i32)) -> HashSet<(i32, i32)> {
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;
    let mut guard = guard_start;
    let mut path = HashSet::new();
    path.insert(guard);

    loop {
        let (delta_row, delta_col) = directions[direction % 4];
        let next_row = guard.0 + delta_row;
        let next_col = guard.1 + delta_col;

        match check_cell(&grid, next_row, next_col) {
            CheckCellResult::Invalid => break,
            CheckCellResult::Obstacle => direction += 1,
            CheckCellResult::Valid => {
                guard = (next_row, next_col);
                path.insert(guard);
            }
        }
    }

    path
}

fn check_loop(grid: &Vec<Vec<char>>, mut guard: (i32, i32)) -> bool {
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;
    let mut visited = HashSet::new();
    let mut consecutive_rotations = 0;

    loop {
        let state = (guard, direction % 4);
        if visited.contains(&state) {
            return true;
        }
        visited.insert(state);

        let (delta_row, delta_col) = directions[direction % 4];
        let next_row = guard.0 + delta_row;
        let next_col = guard.1 + delta_col;

        match check_cell(&grid, next_row, next_col) {
            CheckCellResult::Invalid => return false,
            CheckCellResult::Obstacle => {
                direction += 1;
                consecutive_rotations += 1;
                if consecutive_rotations >= 4 {
                    return false; // Guard is trapped in place
                }
            }
            CheckCellResult::Valid => {
                guard = (next_row, next_col);
                consecutive_rotations = 0;
            }
        }
    }
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
