use std::{io::BufRead, str::FromStr};

use rustc_hash::FxHashMap;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

#[derive(Debug)]
struct Rule(usize, usize);

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('|').expect("broken rule format");

        let left = left.parse().expect("broken left number");
        let right = right.parse().expect("broken left number");

        Ok(Self(left, right))
    }
}

struct Update {
    inner: FxHashMap<usize, usize>,
}

impl Update {
    fn new(inner: impl IntoIterator<Item = usize>) -> Self {
        let inner = inner
            .into_iter()
            .enumerate()
            .map(|(idx, number)| (number, idx))
            .collect();

        Self { inner }
    }

    fn is_conformant_to(&self, rule: &Rule) -> bool {
        let Some(pos_0) = self.inner.get(&rule.0) else {
            return true;
        };

        let Some(pos_1) = self.inner.get(&rule.1) else {
            return true;
        };

        pos_0 < pos_1
    }

    fn apply(&mut self, rule: &Rule) -> bool {
        let Some(pos_0) = self.inner.get(&rule.0).copied() else {
            return false;
        };

        let Some(pos_1) = self.inner.get(&rule.1).copied() else {
            return false;
        };

        if pos_0 > pos_1 {
            self.inner.insert(rule.0, pos_1);
            self.inner.insert(rule.1, pos_0);

            return true;
        }

        false
    }

    fn middle(&self) -> usize {
        let mut reconstructed = self
            .inner
            .iter()
            .map(|(number, idx)| (idx, number))
            .collect::<Vec<_>>();

        reconstructed.sort_unstable_by_key(|(idx, _)| **idx);

        *reconstructed[reconstructed.len() / 2].1
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut lines = lines.map(|line| line.expect("broken line"));

    let mut rules = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        rules.push(line.parse::<Rule>().expect("error parsing rule"));
    }

    let mut updates = vec![];

    for line in lines.by_ref() {
        updates.push(Update::new(
            line.split(',')
                .map(|n| n.parse::<usize>().expect("error parsing update")),
        ));
    }

    updates
        .into_iter()
        .filter(|update| rules.iter().any(|rule| !update.is_conformant_to(rule)))
        .map(|mut update| {
            loop {
                if rules.iter().filter(|rule| update.apply(rule)).count() == 0 {
                    break;
                }
            }

            update
        })
        .map(|update| update.middle())
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(123, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(5353, solve(std::io::BufReader::new(file).lines()));
}