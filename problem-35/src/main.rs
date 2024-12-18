use std::{collections::VecDeque, io::BufRead};

use rustc_hash::FxHashSet;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

static DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn diff(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    fn next(&self, direction: Direction, max_y: usize, max_x: usize) -> Option<Position> {
        let (dy, dx) = direction.diff();

        self.next_diff(dy, dx, max_y, max_x)
    }

    fn next_diff(&self, dy: isize, dx: isize, max_y: usize, max_x: usize) -> Option<Position> {
        let y = self.y as isize + dy;
        let x = self.x as isize + dx;

        if y < 0 || y as usize > max_y {
            return None;
        }

        if x < 0 || x as usize > max_x {
            return None;
        }

        Some(Position::new(y as usize, x as usize))
    }
}

#[derive(Debug)]
struct Field {
    max_y: usize,
    max_x: usize,
}

impl Field {
    fn new(max_y: usize, max_x: usize) -> Self {
        Self { max_y, max_x }
    }

    fn count_steps(&self, start: Position, end: Position, corrupt: &FxHashSet<Position>) -> usize {
        let mut seen = FxHashSet::default();
        seen.insert(start);

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((position, count)) = queue.pop_front() {
            if position == end {
                return count;
            }

            for direction in DIRECTIONS {
                if let Some(next) = position.next(*direction, self.max_y, self.max_x) {
                    if corrupt.contains(&next) {
                        continue;
                    }

                    if !seen.insert(next) {
                        continue;
                    }

                    queue.push_back((next, count + 1));
                }
            }
        }

        0
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut max_y = 0;
    let mut max_x = 0;

    let corrupt = lines
        .map(|line| {
            let line = line.expect("broken line");

            let (x, y) = line.split_once(',').expect("broken coordinate format");

            let y = y.parse().expect("error parsing y");
            let x = x.parse().expect("error parsing y");

            max_y = max_y.max(y);
            max_x = max_x.max(x);

            Position::new(y, x)
        })
        .collect::<Vec<_>>();

    let corrupt = if max_y == 6 && max_x == 6 {
        corrupt.into_iter().take(12)
    } else {
        corrupt.into_iter().take(1024)
    }
    .collect::<FxHashSet<_>>();

    let field = Field::new(max_y, max_x);

    field.count_steps(Position::new(0, 0), Position::new(max_y, max_x), &corrupt)
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(22, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(360, solve(std::io::BufReader::new(file).lines()));
}
