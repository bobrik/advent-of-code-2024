use std::{io::BufRead, str::FromStr};

use z3::ast::Ast;

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
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
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

        let x = x[2..].parse::<usize>().expect("error parsing x");
        let y = y[2..].parse::<usize>().expect("error parsing x");

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
        let ctx = z3::Context::new(&z3::Config::new());

        let solver = z3::Solver::new(&ctx);

        let a = z3::ast::Int::new_const(&ctx, "a");
        let b = z3::ast::Int::new_const(&ctx, "b");

        let a_x = z3::ast::Int::from_u64(&ctx, self.button_a.x as u64);
        let a_y = z3::ast::Int::from_u64(&ctx, self.button_a.y as u64);

        let b_x = z3::ast::Int::from_u64(&ctx, self.button_b.x as u64);
        let b_y = z3::ast::Int::from_u64(&ctx, self.button_b.y as u64);

        let p_x = z3::ast::Int::from_u64(&ctx, self.prize.x as u64);
        let p_y = z3::ast::Int::from_u64(&ctx, self.prize.y as u64);

        solver.assert(&(a.clone() * a_x + b.clone() * b_x)._eq(&p_x));
        solver.assert(&(a.clone() * a_y + b.clone() * b_y)._eq(&p_y));

        if solver.check() != z3::SatResult::Sat {
            return None;
        }

        let model = solver.get_model().expect("error getting solver model");

        let a = model
            .eval(&a, true)
            .expect("error evaluating a")
            .as_u64()
            .expect("error representing a as u64") as usize;

        let b = model
            .eval(&b, true)
            .expect("error evaluating b")
            .as_u64()
            .expect("error representing b as u64") as usize;

        Some(a * 3 + b)
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
