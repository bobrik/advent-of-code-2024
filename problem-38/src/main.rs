use std::io::BufRead;

use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

fn count_ways(
    design: &str,
    patterns: &FxHashSet<&str>,
    max_pattern_len: usize,
    counts: &mut FxHashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = counts.get(design) {
        return *count;
    }

    let mut count = 0;

    let cap = max_pattern_len.min(design.len());

    for prefix_len in 1..=cap {
        let prefix = &design[..prefix_len];
        if !patterns.contains(prefix) {
            continue;
        }

        count += count_ways(&design[prefix_len..], patterns, max_pattern_len, counts);
    }

    counts.insert(design.to_string(), count);

    count
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    let patterns = lines
        .next()
        .expect("missing patterns line")
        .expect("broken patterns line");

    let patterns = patterns.split(", ").collect::<FxHashSet<_>>();

    let max_pattern_len = patterns
        .iter()
        .map(|pattern| pattern.len())
        .max()
        .expect("no patterns");

    let designs = lines
        .filter_map(|line| {
            let line = line.expect("broken line");

            if line.is_empty() {
                return None;
            }

            Some(line)
        })
        .collect::<Vec<_>>();

    let mut counts = FxHashMap::default();

    designs
        .iter()
        .map(|design| count_ways(design, &patterns, max_pattern_len, &mut counts))
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(16, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(278, solve(std::io::BufReader::new(file).lines()));
}
