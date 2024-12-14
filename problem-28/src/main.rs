use std::{io::BufRead, str::FromStr};

use rustc_hash::{FxBuildHasher, FxHashSet};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    y: isize,
    x: isize,
}

impl Position {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

#[derive(Debug)]
struct Velocity {
    dy: isize,
    dx: isize,
}

impl Velocity {
    fn new(dy: isize, dx: isize) -> Self {
        Self { dy, dx }
    }
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    fn move_by(&mut self, steps: usize) {
        self.position.x = (self.position.x + self.velocity.dx * steps as isize).rem_euclid(WIDTH);
        self.position.y = (self.position.y + self.velocity.dy * steps as isize).rem_euclid(HEIGHT);
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(' ').expect("broken robot line format");

        let position = position
            .strip_prefix("p=")
            .expect("missing position prefix");

        let (x, y) = position.split_once(',').expect("broken position format");

        let x = x.parse().expect("error parsing position x");
        let y = y.parse().expect("error parsing position y");

        let position = Position::new(y, x);

        let velocity = velocity
            .strip_prefix("v=")
            .expect("missing velocity prefix");

        let (dx, dy) = velocity.split_once(',').expect("broken velocity format");

        let dx = dx.parse().expect("error parsing velocity dx");
        let dy = dy.parse().expect("error parsing velocity dy");

        let velocity = Velocity::new(dy, dx);

        Ok(Robot::new(position, velocity))
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut robots = lines
        .map(|line| {
            line.expect("broken line")
                .parse::<Robot>()
                .expect("error parsing robot")
        })
        .collect::<Vec<_>>();

    let mut seen = FxHashSet::with_capacity_and_hasher(robots.len(), FxBuildHasher);

    for step in 1.. {
        for robot in robots.iter_mut() {
            robot.move_by(1);
        }

        seen.clear();

        let mut found_duplicates = false;

        for robot in robots.iter() {
            if !seen.insert(robot.position) {
                found_duplicates = true;
                break;
            }
        }

        if !found_duplicates {
            return step;
        }
    }

    unreachable!()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(1, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(8159, solve(std::io::BufReader::new(file).lines()));
}
