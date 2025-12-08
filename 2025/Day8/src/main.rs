use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn distance_from(&self, other: &Point) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn update_graph(
    locations: &Vec<Point>,
    pairs_visited: &mut HashSet<(usize, usize)>,
    adj: &mut Vec<Vec<usize>>,
    iterations: usize,
) -> Option<(usize, usize)> {
    let n = locations.len();
    let mut last_made_conn = None;

    for _ in 0..iterations {
        let mut lowest_dist = f64::MAX;
        let mut best_pair: Option<(usize, usize)> = None;

        for i in 0..n {
            let p1 = locations[i];
            for j in (i + 1)..n {
                if pairs_visited.contains(&(i, j)) {
                    continue;
                }
                let p2 = locations[j];
                let dist = p1.distance_from(&p2);
                if dist < lowest_dist {
                    lowest_dist = dist;
                    best_pair = Some((i, j));
                }
            }
        }

        if let Some((a, b)) = best_pair {
            pairs_visited.insert((a, b));
            adj[a].push(b);
            adj[b].push(a);
            last_made_conn = Some((a, b));
        }
    }

    last_made_conn
}

fn solve_part1(locations: Vec<Point>) -> usize {
    const TOTAL_ITERATIONS: usize = 1000;
    let n = locations.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut pairs_visited: HashSet<(usize, usize)> = HashSet::new();

    update_graph(&locations, &mut pairs_visited, &mut adj, TOTAL_ITERATIONS);

    let mut visited = vec![false; n];
    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut sizes: Vec<usize> = Vec::new();

    for start in 0..n {
        if visited[start] {
            continue;
        }
        let mut size = 0;
        stack.push_front(start);
        while let Some(idx) = stack.pop_front() {
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            size += 1;
            for &nei in &adj[idx] {
                stack.push_front(nei);
            }
        }
        if size > 0 {
            sizes.push(size);
        }
    }

    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes[..3].iter().fold(1, |acc, x| acc * x)
}

fn solve_part2(locations: Vec<Point>) -> usize {
    let n = locations.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut pairs_visited: HashSet<(usize, usize)> = HashSet::new();

    let mut visited = vec![false; n];
    let mut stack: VecDeque<usize> = VecDeque::new();

    loop {
        let last = update_graph(&locations, &mut pairs_visited, &mut adj, 1).unwrap();
        visited.fill(false);
        let mut size = 0;
        stack.clear();
        stack.push_front(0);
        while let Some(idx) = stack.pop_front() {
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            size += 1;
            for &nei in &adj[idx] {
                stack.push_front(nei);
            }
        }
        if size == n {
            let (i, j) = last;
            return locations[i].x * locations[j].x;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let locations: Vec<Point> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let coords: Vec<&str> = line.trim().split(',').collect();
            let x: usize = coords[0].parse().unwrap();
            let y: usize = coords[1].parse().unwrap();
            let z: usize = coords[2].parse().unwrap();
            Point { x, y, z }
        })
        .collect();

    println!("Part1: {}", solve_part1(locations.clone()));
    println!("Part2: {}", solve_part2(locations));

    Ok(())
}
