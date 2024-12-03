use std::{io::BufRead, str::FromStr};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

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
            .ok_or_else(|| "missing 'mul(' prefix".to_owned())?
            .strip_suffix(')')
            .ok_or_else(|| "missing ')' suffix".to_owned())?
            .split_once(',')
            .ok_or_else(|| "broken mul(...)".to_owned())?;

        let one = one.parse::<usize>().map_err(|error| error.to_string())?;
        let two = two.parse::<usize>().map_err(|error| error.to_string())?;

        Ok(Self { one, two })
    }
}

fn compute(mut input: &str) -> usize {
    let mut sum = 0;

    loop {
        input = match input.find("mul(") {
            Some(start) => match &input[start..].find(')') {
                Some(end) => match &input[start..start + end + 1].parse::<Multiplication>() {
                    Ok(multiplication) => {
                        sum += multiplication.compute();
                        &input[start + end + 1..]
                    }
                    Err(_) => &input[start + 1..],
                },
                None => return sum,
            },
            None => return sum,
        };
    }
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    let line = lines.next().expect("missing line").expect("broken line");

    let mut sum = 0;
    let mut remaining = &line[..];

    loop {
        remaining = match remaining.find("don't") {
            Some(end) => {
                sum += compute(&remaining[..end]);

                match remaining[end + 2..].find("do") {
                    Some(start) => &remaining[end + 2 + start..],
                    None => return sum + compute(remaining),
                }
            }
            None => return sum + compute(remaining),
        };
    }
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(48, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(56275602, solve(std::io::BufReader::new(file).lines()));
}
