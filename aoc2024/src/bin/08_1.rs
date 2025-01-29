use std::collections::{HashMap, HashSet};

fn main() {
    let result = solve(include_str!("../../input/08.txt"));
    println!("{}", result)
}

fn solve(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut antennas = HashMap::new();
    let mut antinodes = HashSet::new();

    // Collect all antennas
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != '.' {
                antennas
                    .entry(grid[i][j])
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        }
    }

    // Check each group of antennas with the same symbol
    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (ay, ax) = positions[i];
                let (by, bx) = positions[j];

                let dy = by - ay;
                let dx = bx - ax;

                // Add antinodes to both sides
                let forward = (by + dy, bx + dx);
                let backward = (ay - dy, ax - dx);

                // Insert if within grid bounds
                if valid_position(forward.0, forward.1, rows, cols) {
                    antinodes.insert(forward);
                }
                if valid_position(backward.0, backward.1, rows, cols) {
                    antinodes.insert(backward);
                }
            }
        }
    }

    antinodes.len() as i32
}

fn valid_position(row: i32, col: i32, rows: usize, cols: usize) -> bool {
    row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
}
