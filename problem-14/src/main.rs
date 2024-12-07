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

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Concatenate,
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operator::Sum => left + right,
            Operator::Product => left * right,
            Operator::Concatenate => left * 10usize.pow(right.ilog10() + 1) + right,
        }
    }
}

struct Combinator<T>
where
    T: 'static,
{
    len: usize,
    values: &'static [T],
    current: Vec<usize>,
    computed: Vec<&'static T>,
}

impl<T> Combinator<T> {
    fn new(values: &'static [T], len: usize) -> Self {
        let current = vec![0; len];
        let computed = current.iter().map(|idx| &values[*idx]).collect();

        Self {
            len,
            values,
            current,
            computed,
        }
    }

    fn advance(&mut self) -> bool {
        for i in 0..self.len {
            if self.current[i] == self.values.len() - 1 {
                if i == self.len - 1 {
                    return false;
                }

                self.current[i] = 0;

                continue;
            }

            self.current[i] += 1;

            for idx in 0..self.len {
                self.computed[idx] = &self.values[self.current[idx]];
            }

            return true;
        }

        false
    }

    fn current(&self) -> &[&T] {
        &self.computed
    }
}

#[derive(Debug)]
struct Equation {
    result: usize,
    constituents: Vec<usize>,
}

impl Equation {
    fn can_be_solved(&self) -> bool {
        let mut combinator = Combinator::new(OPERATORS, self.constituents.len() - 1);

        loop {
            let mut accumulator = self.constituents[0];

            for (idx, operator) in combinator.current().iter().enumerate() {
                if accumulator > self.result {
                    break;
                }

                accumulator = operator.apply(accumulator, self.constituents[idx + 1]);
            }

            if accumulator == self.result {
                return true;
            }

            if !combinator.advance() {
                break;
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
