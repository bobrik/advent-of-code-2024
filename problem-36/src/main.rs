use std::{cmp::Ordering, collections::VecDeque, io::BufRead};

use bitvec::{bitvec, vec::BitVec};

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

    fn bit(&self, max_x: usize) -> usize {
        self.y * (max_x + 1) + self.x
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

    fn is_reachable_with(&self, start: Position, end: Position, corrupt: &BitVec) -> bool {
        let mut seen = bitvec![0; (self.max_y + 1) * (self.max_x + 1)];
        seen.set(start.bit(self.max_x), true);

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(position) = queue.pop_front() {
            if position == end {
                return true;
            }

            for direction in DIRECTIONS {
                if let Some(next) = position.next(*direction, self.max_y, self.max_x) {
                    if corrupt[next.bit(self.max_x)] {
                        continue;
                    }

                    if seen.replace(next.bit(self.max_x), true) {
                        continue;
                    }

                    queue.push_back(next);
                }
            }
        }

        false
    }

    fn first_death(&self, start: Position, end: Position, candidates: &[Position]) -> Position {
        let first_idx = (0..candidates.len())
            .collect::<Vec<_>>()
            .binary_search_by(|idx| {
                let mut corrupt = bitvec![0; (self.max_y + 1) * (self.max_x + 1)];
                for position in &candidates[..=*idx] {
                    corrupt.set(position.bit(self.max_x), true);
                }

                if self.is_reachable_with(start, end, &corrupt) {
                    return Ordering::Less;
                }

                Ordering::Greater
            })
            .expect_err("no corruption blocks the path");

        candidates[first_idx]
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> String {
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

    let field = Field::new(max_y, max_x);

    let first = field.first_death(Position::new(0, 0), Position::new(max_y, max_x), &corrupt);

    format!("{},{}", first.x, first.y)
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!("6,1", solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!("58,62", solve(std::io::BufReader::new(file).lines()));
}
