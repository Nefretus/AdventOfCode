use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Range {
    low: u64,
    high: u64,
}

fn solve_part1(ranges: &Vec<Range>, ids: &Vec<u64>) -> u64 {
    let mut fresh_count = 0;
    for id in ids {
        for range in ranges {
            if *id >= range.low && *id <= range.high {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn solve_part2(mut ranges: Vec<Range>) -> u64 {
    ranges.sort_by_key(|r| r.low);
    let mut merged: Vec<Range> = Vec::new();
    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.low <= last.high {
                if r.high > last.high {
                    last.high = r.high;
                }
                continue;
            }
        }
        merged.push(r);
    }
    merged
        .iter()
        .fold(0, |acc, range| acc + range.high - range.low + 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut lines = input.lines();

    let ranges: Vec<Range> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (low, high) = line.split_once('-').unwrap();
            Range {
                low: low.parse().unwrap(),
                high: high.parse().unwrap(),
            }
        })
        .collect();

    let ids: Vec<u64> = lines.map(|e| e.parse().unwrap()).collect();

    println!("Part1: {}", solve_part1(&ranges, &ids));
    println!("Part2: {}", solve_part2(ranges));

    Ok(())
}
