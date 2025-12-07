use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

const START: char = 'S';
const SPLITTER: char = '^';
const EMPTY: char = '.';

fn solve_part1(
    grid: &Vec<Vec<char>>,
    used_splitters_coords: &mut HashSet<(usize, usize)>,
    row: usize,
    col: usize,
) {
    let mut curr_row = row;
    while grid[curr_row][col] != SPLITTER {
        curr_row += 1;
        if curr_row >= grid.len() {
            return;
        }
    }
    if !used_splitters_coords.insert((curr_row, col)) {
        return;
    }
    if col + 1 < grid[0].len() {
        solve_part1(grid, used_splitters_coords, curr_row, col + 1);
    }
    if col > 0 {
        solve_part1(grid, used_splitters_coords, curr_row, col - 1);
    }
}

fn solve_part2(
    cache: &mut HashMap<(usize, usize), usize>,
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> usize {
    if let Some(&cost) = cache.get(&(row, col)) {
        return cost;
    }
    let cost = if row >= grid.len() {
        1
    } else if grid[row][col] == EMPTY || grid[row][col] == START {
        solve_part2(cache, grid, row + 1, col)
    } else {
        solve_part2(cache, grid, row, col - 1) + solve_part2(cache, grid, row, col + 1)
    };
    cache.insert((row, col), cost);
    cost
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut used_splitters_coords: HashSet<(usize, usize)> = HashSet::new();
    let start_col = grid
        .first()
        .unwrap()
        .iter()
        .position(|&c| c == START)
        .unwrap();

    solve_part1(&grid, &mut used_splitters_coords, 0, start_col);
    println!("Part1: {}", used_splitters_coords.len());
    println!(
        "Part2: {}",
        solve_part2(&mut HashMap::new(), &grid, 0, start_col)
    );

    Ok(())
}
