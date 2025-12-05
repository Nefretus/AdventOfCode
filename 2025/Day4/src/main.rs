use std::error::Error;
use std::fs;

const EMPTY: char = '.';
const ROLL_OF_PAPER: char = '@';

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn adjacent_count_bounded(grid: &[Vec<char>], row: usize, col: usize, limit: u32) -> u32 {
    let rows = grid.len() as isize;
    let cols = grid.first().unwrap().len() as isize;
    let mut count = 0;
    for (dr, dc) in &DIRECTIONS {
        let nr = row as isize + dr;
        let nc = col as isize + dc;
        if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            if grid[nr as usize][nc as usize] == ROLL_OF_PAPER {
                count += 1;
                if count > limit {
                    break;
                }
            }
        }
    }
    count
}

fn positions_with_at_most_adjacent(grid: &[Vec<char>], max_adj: u32) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (r, row_vec) in grid.iter().enumerate() {
        for (c, &cell) in row_vec.iter().enumerate() {
            if cell == EMPTY {
                continue;
            }
            if adjacent_count_bounded(grid, r, c, max_adj) <= max_adj {
                result.push((r, c));
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut layout: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let available_now = positions_with_at_most_adjacent(&layout, 3).len();
    println!("Part 1: {}", available_now);

    let mut removed_total = 0;
    loop {
        let to_remove = positions_with_at_most_adjacent(&layout, 3);
        if to_remove.is_empty() {
            break;
        }
        removed_total += to_remove.len();
        for (r, c) in to_remove {
            layout[r][c] = EMPTY;
        }
    }

    println!("Part 2: {}", removed_total);
    Ok(())
}
