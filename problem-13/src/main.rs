use std::{io::BufRead, str::FromStr};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

enum Operator {
    Sum,
    Product,
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operator::Sum => left + right,
            Operator::Product => left * right,
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: usize,
    constituents: Vec<usize>,
}

impl Equation {
    fn can_be_solved(&self) -> bool {
        for n in 0..2usize.pow(self.constituents.len() as u32 - 1) {
            let mut accumulator = self.constituents[0];

            for idx in 1..self.constituents.len() {
                if accumulator > self.result {
                    break;
                }

                let operator = if n & 1 << (idx - 1) != 0 {
                    Operator::Product
                } else {
                    Operator::Sum
                };

                accumulator = operator.apply(accumulator, self.constituents[idx])
            }

            if accumulator == self.result {
                return true;
            }
        }

        false
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, constituents) = s.split_once(": ").expect("broken equation format");

        let result = result.parse().expect("error parsing result");
        let constituents = constituents
            .split_ascii_whitespace()
            .map(|n| n.parse().expect("error parsing constituent"))
            .collect();

        Ok(Self {
            result,
            constituents,
        })
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    lines
        .map(|line| {
            line.expect("broken line")
                .parse::<Equation>()
                .expect("error parsing equation")
        })
        .filter(|equation| equation.can_be_solved())
        .map(|equation| equation.result)
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(3749, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(303766880536, solve(std::io::BufReader::new(file).lines()));
}
