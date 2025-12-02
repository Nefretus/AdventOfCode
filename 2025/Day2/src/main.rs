use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    begin: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.trim().split_once('-').ok_or("missing '-'")?;
        Ok(Range {
            begin: a.parse()?,
            end: b.parse()?,
        })
    }
}

fn count_part1(val: u64) -> u64 {
    let s = val.to_string();
    let n_digits = s.len();
    if n_digits & 1 == 0 {
        let half = n_digits / 2;
        let (left, right) = s.split_at(half);
        if left == right {
            return val as u64;
        }
    }
    0
}

thread_local! {
    static CACHE: RefCell<HashMap<usize, Vec<usize>>> = RefCell::new(HashMap::new());
}

fn divisors(n: usize) -> Vec<usize> {
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();

        if let Some(v) = cache.get(&n) {
            return v.clone();
        }

        let mut out = Vec::new();
        let mut i = 1;
        while i * i <= n {
            if n % i == 0 {
                if i != n {
                    out.push(i);
                }
                let div = n / i;
                if div != n && div != i {
                    out.push(div);
                }
            }
            i += 1;
        }

        out.sort_unstable();
        cache.insert(n, out.clone());
        out
    })
}

fn count_part2(val: u64) -> u64 {
    let s = val.to_string();
    let n_digits = s.len();

    for unit_len in divisors(n_digits) {
        let repeats = n_digits / unit_len;
        let unit = &s[..unit_len];
        if (1..repeats).all(|k| &s[k * unit_len..(k + 1) * unit_len] == unit) {
            return val;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let ranges: Vec<Range> = input
        .split(',')
        .map(|s| s.parse::<Range>())
        .collect::<Result<_, _>>()?;

    let mut count_invalid_part1: u64 = 0;
    let mut count_invalid_part2: u64 = 0;

    for range in ranges {
        for val in range.begin..=range.end {
            count_invalid_part1 += count_part1(val);
            count_invalid_part2 += count_part2(val);
        }
    }

    println!("part1: {}, part2: {}", count_invalid_part1, count_invalid_part2);
    Ok(())
}
