use std::{cmp::Ordering, io::BufRead};

use itertools::Itertools;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

fn is_good<'a, I>(input: I) -> bool
where
    I: Iterator<Item = &'a isize>,
{
    let mut order = Ordering::Equal;

    for (prev, next) in input.tuple_windows() {
        if order == Ordering::Equal {
            order = if next > prev {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }

        let diff = next - prev;

        if !match order {
            Ordering::Less => (-3..=-1).contains(&diff),
            Ordering::Equal => unreachable!(),
            Ordering::Greater => (1..=3).contains(&diff),
        } {
            return false;
        }
    }

    true
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    lines
        .map(|line| line.expect("broken line"))
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<isize>().expect("broken number"))
                .collect::<Vec<_>>()
        })
        .filter(|row| is_good(row.iter()))
        .count()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(2, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(202, solve(std::io::BufReader::new(file).lines()));
}
