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
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, position) = s.split_once(": ").expect("broken position / offset line");

        let (x, y) = position
            .split_once(", ")
            .expect("broken position / offset format");

        let x = x[2..].parse::<isize>().expect("error parsing x");
        let y = y[2..].parse::<isize>().expect("error parsing x");

        Ok(Position::new(x, y))
    }
}

#[derive(Debug)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Machine {
    fn new(button_a: Position, button_b: Position, prize: Position) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn cheapest_option(&self) -> Option<usize> {
        let b = (self.prize.x * self.button_a.y - self.prize.y * self.button_a.x)
            / (self.button_a.y * self.button_b.x - self.button_b.y * self.button_a.x);

        let a = (self.prize.x * self.button_b.y - self.prize.y * self.button_b.x)
            / (self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y);

        if a * self.button_a.x + b * self.button_b.x != self.prize.x {
            return None;
        }

        if a * self.button_a.y + b * self.button_b.y != self.prize.y {
            return None;
        }

        Some(a as usize * 3 + b as usize)
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut lines = lines.map(|line| line.expect("broken line"));

    let mut machines = vec![];

    loop {
        let Some(line) = lines.next() else {
            break;
        };

        if line.is_empty() {
            continue;
        }

        let button_a = line.parse().expect("error parsing button a line");

        let button_b = lines
            .next()
            .expect("missing button b line")
            .parse()
            .expect("error parsing button b line");

        let prize = lines
            .next()
            .expect("missing prize line")
            .parse::<Position>()
            .expect("error parsing prize line");

        let prize = Position::new(prize.x + 10000000000000, prize.y + 10000000000000);

        machines.push(Machine::new(button_a, button_b, prize));
    }

    machines
        .iter()
        .filter_map(|machine| machine.cheapest_option())
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(875318608908, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(95688837203288, solve(std::io::BufReader::new(file).lines()));
}
