use std::error::Error;
use std::fs;

const MULTIPLICATION: char = '*';
const ADDITION: char = '+';

fn solve_part1(value_lines: Vec<String>, ops_line: &str) -> u64 {
    let values: Vec<Vec<u64>> = value_lines
        .iter()
        .map(|ln| {
            ln.split_whitespace()
                .map(|tok| tok.parse().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<char> = ops_line
        .chars()
        .filter(|&ch| ch == MULTIPLICATION || ch == ADDITION)
        .collect();

    let mut grand_total = 0;
    for col in 0..ops.len() {
        let mut sum = values[0][col];
        for row in 1..values.len() {
            if ops[col] == MULTIPLICATION {
                sum *= values[row][col];
            }
            if ops[col] == ADDITION {
                sum += values[row][col];
            }
        }
        grand_total += sum;
    }
    grand_total
}

fn solve_part2(grid: &Vec<Vec<char>>, ops_line: &str) -> u64 {
    let width = ops_line.len();

    let is_separator_col = |col: usize| -> bool { grid.iter().all(|row| row[col] == ' ') };

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut col = 0;
    while col < width {
        if is_separator_col(col) {
            col += 1;
            continue;
        }
        let start = col;
        while col < width && !is_separator_col(col) {
            col += 1;
        }
        let end = col - 1;
        blocks.push((start, end));
    }

    let ops: Vec<char> = ops_line
        .chars()
        .filter(|&ch| ch == MULTIPLICATION || ch == ADDITION)
        .collect();

    let mut grand_total: u64 = 0;

    for i in 0..ops.len() {
        let (left, right) = blocks[i];
        let mut numbers: Vec<u64> = Vec::new();
        for col in (left..=right).rev() {
            let mut num: u64 = 0;
            let mut saw_digit = false;
            for row in 0..grid.len() {
                let c = grid[row][col];
                if c.is_ascii_digit() {
                    saw_digit = true;
                    num = num * 10 + (c.to_digit(10).unwrap() as u64);
                }
            }
            if saw_digit {
                numbers.push(num);
            }
        }
        let mut acc = numbers[0];
        for &val in &numbers[1..] {
            match ops[i] {
                MULTIPLICATION => acc *= val,
                ADDITION => acc += val,
                _ => {}
            }
        }
        grand_total += acc;
    }

    grand_total
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    
    let ops_line = lines.pop().unwrap();
    let grid: Vec<Vec<char>> = lines.clone().into_iter().map(|s| s.chars().collect()).collect();

    println!("Part1: {}", solve_part1(lines, &ops_line));
    println!("Part2: {}", solve_part2(&grid, &ops_line));

    Ok(())
}
