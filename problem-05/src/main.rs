use std::{io::BufRead, str::FromStr, sync::LazyLock};

use regex::Regex;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

static MUL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"mul\(\d{1,3},\d{1,3}\)"#).expect("error compiling regexp"));

struct Multiplication {
    one: usize,
    two: usize,
}

impl Multiplication {
    fn compute(&self) -> usize {
        self.one * self.two
    }
}

impl FromStr for Multiplication {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two) = s
            .strip_prefix("mul(")
            .expect("missing 'mul(' prefix")
            .strip_suffix(')')
            .expect("missing ')' suffix")
            .split_once(',')
            .expect("broken mul(...)");

        let one = one.parse().expect("broken left operand");
        let two = two.parse().expect("broken right operand");

        Ok(Self { one, two })
    }
}

fn compute(input: &str) -> usize {
    MUL_REGEX
        .find_iter(input)
        .map(|m| {
            m.as_str()
                .parse::<Multiplication>()
                .expect("error parsing multiplication")
        })
        .map(|m| m.compute())
        .sum()
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    compute(&lines.next().expect("missing line").expect("broken line"))
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(161, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(174960292, solve(std::io::BufReader::new(file).lines()));
}
