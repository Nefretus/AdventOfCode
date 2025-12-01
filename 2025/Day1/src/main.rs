const DIAL_SIZE: i32 = 100;

fn count_zero_hits_during_move(start: i32, delta: i32) -> i32 {
    if delta == 0 {
        return 0;
    }
    let steps = delta.abs();
    let rem = if delta > 0 {
        DIAL_SIZE - start
    } else {
        if start == 0 { DIAL_SIZE } else { start }
    };
    if steps < rem {
        return 0;
    }
    1 + (steps - rem) / DIAL_SIZE
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let values: Vec<i32> = input
        .lines()
        .map(|line| {
            let n: i32 = line[1..].trim().parse().unwrap();
            if line.chars().next().unwrap() == 'L' {
                -n
            } else {
                n
            }
        })
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut pos = 50;

    for delta in values {
        part2 += count_zero_hits_during_move(pos, delta);

        let mut res = pos + delta;
        while res < 0 {
            res += DIAL_SIZE;
        }
        while res >= DIAL_SIZE {
            res -= DIAL_SIZE;
        }

        pos = res;

        if pos == 0 {
            part1 += 1;
        }
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    Ok(())
}
