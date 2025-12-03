use std::error::Error;
use std::fs;
use std::ptr;

fn solve_part1(banks: &Vec<Vec<u64>>) -> u64 {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        let mut first_digit = 0;
        let mut second_digit = 0;
        if let Some(last_e) = bank.last() {
            for e in bank.iter() {
                if *e > first_digit && !ptr::eq(e, last_e) {
                    first_digit = *e;
                    second_digit = 0;
                } else if *e > second_digit {
                    second_digit = *e;
                }
            }
            total_joltage += first_digit * 10 + second_digit;
        }
    }
    total_joltage
}

fn solve_part2(banks: &Vec<Vec<u64>>) -> u64 {
    let total_bank_length = banks.first().unwrap().len();
    const BANK_BAT: usize = 12;
    let mut total_joltage: u64 = 0;
    for bank in banks {
        let mut out_vec: Vec<u64> = vec![0; BANK_BAT];
        let mut start_idx = 0;
        let mut best_idx = 0;
        let mut skip_space = total_bank_length - BANK_BAT + 1;
        for (idx, val) in out_vec.iter_mut().enumerate() {
            if skip_space > 0 {
                for i in start_idx..(start_idx + skip_space) {
                    if *val < bank[i] {
                        best_idx = i;
                        *val = bank[i];
                    }
                }
                skip_space -= (best_idx - start_idx);
                start_idx = best_idx + 1;
            }
            else {
                *val = bank[idx];
            }
        }
        total_joltage += out_vec
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                let exp = (BANK_BAT - idx - 1) as u32;
                (*val) * 10u64.pow(exp)
            })
            .sum::<u64>();
    }
    total_joltage
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let banks: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    println!("Part1: {}\nPart2: {}", solve_part1(&banks), solve_part2(&banks));
    Ok(())
}
