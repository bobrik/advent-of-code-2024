use std::{io::BufRead, str::FromStr};

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
    inner: [Option<usize>; 256],
}

impl Update {
    fn new(input: impl IntoIterator<Item = usize>) -> Self {
        let mut inner = [None; 256];

        for (idx, number) in input.into_iter().enumerate() {
            inner[number] = Some(idx);
        }

        Self { inner }
    }

    fn is_conformant_to(&self, rule: &Rule) -> bool {
        let Some(pos_0) = self.inner[rule.0] else {
            return true;
        };

        let Some(pos_1) = self.inner[rule.1] else {
            return true;
        };

        pos_0 < pos_1
    }

    fn middle(&self) -> usize {
        let mut reconstructed = self
            .inner
            .iter()
            .enumerate()
            .filter_map(|(number, idx)| idx.map(|idx| (idx, number)))
            .collect::<Vec<_>>();

        reconstructed.sort_unstable_by_key(|(idx, _)| *idx);

        reconstructed[reconstructed.len() / 2].1
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
        .filter(|update| rules.iter().all(|rule| update.is_conformant_to(rule)))
        .map(|update| update.middle())
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(143, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(6384, solve(std::io::BufReader::new(file).lines()));
}
