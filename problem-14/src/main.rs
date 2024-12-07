use std::{io::BufRead, str::FromStr};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

static OPERATORS: &[Operator] = &[Operator::Sum, Operator::Product, Operator::Concatenate];

#[derive(Clone, Copy)]
enum Operator {
    Sum,
    Product,
    Concatenate,
}

impl Operator {
    fn apply_inverse(&self, left: usize, right: usize) -> Option<usize> {
        match self {
            Operator::Sum => {
                if right > left {
                    Some(right - left)
                } else {
                    None
                }
            }
            Operator::Product => {
                if right % left == 0 {
                    Some(right / left)
                } else {
                    None
                }
            }
            Operator::Concatenate => {
                if right == 0 {
                    return None;
                }

                let right_ilog10 = right.ilog10();
                let left_ilog10 = left.ilog10();

                if right_ilog10 >= left_ilog10 && right % 10usize.pow(left_ilog10 + 1) == left {
                    Some(right / 10usize.pow(left_ilog10 + 1))
                } else {
                    None
                }
            }
        }
    }
}

struct Equation {
    result: usize,
    constituents: Vec<usize>,
}

impl Equation {
    fn can_reach_result_via(
        result: usize,
        operator: Operator,
        constituents: &[usize],
        idx: usize,
    ) -> bool {
        let Some(result) = operator.apply_inverse(constituents[idx], result) else {
            return false;
        };

        if idx == 1 {
            return constituents[0] == result;
        }

        for operator in OPERATORS {
            if Self::can_reach_result_via(result, *operator, constituents, idx - 1) {
                return true;
            }
        }

        false
    }

    fn can_be_solved(&self) -> bool {
        for operator in OPERATORS {
            if Self::can_reach_result_via(
                self.result,
                *operator,
                &self.constituents,
                self.constituents.len() - 1,
            ) {
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
    assert_eq!(11387, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(
        337041851384440,
        solve(std::io::BufReader::new(file).lines())
    );
}
