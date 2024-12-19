use std::io::BufRead;

use rustc_hash::FxHashSet;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

fn is_possible(
    design: &str,
    patterns: &FxHashSet<&str>,
    max_pattern_len: usize,
    impossible: &mut FxHashSet<String>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if impossible.contains(design) {
        return false;
    }

    let cap = max_pattern_len.min(design.len());

    for prefix_len in 1..=cap {
        let prefix = &design[..prefix_len];
        if !patterns.contains(prefix) {
            continue;
        }

        if is_possible(&design[prefix_len..], patterns, max_pattern_len, impossible) {
            return true;
        }
    }

    impossible.insert(design.to_string());

    false
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

    let mut impossible = FxHashSet::default();

    designs
        .iter()
        .filter(|design| is_possible(design, &patterns, max_pattern_len, &mut impossible))
        .count()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(6, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(278, solve(std::io::BufReader::new(file).lines()));
}
