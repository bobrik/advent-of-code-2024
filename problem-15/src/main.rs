use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::Itertools;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
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

    fn with_offset(&self, dy: isize, max_y: usize, dx: isize, max_x: usize) -> Option<Position> {
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

    fn targets_with(&self, other: &Position, max_y: usize, max_x: usize) -> Vec<Position> {
        let mut targets = vec![];

        if let Some(position) = Self::with_offset(
            self,
            self.y as isize - other.y as isize,
            max_y,
            self.x as isize - other.x as isize,
            max_x,
        ) {
            targets.push(position);
        }

        if let Some(position) = Self::with_offset(
            other,
            other.y as isize - self.y as isize,
            max_y,
            other.x as isize - self.x as isize,
            max_x,
        ) {
            targets.push(position);
        }

        targets
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Occupancy {
    Vacant,
    Occupied(char),
}

impl Occupancy {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Vacant,
            '0'..='9' | 'a'..='z' | 'A'..='Z' => Self::Occupied(c),
            _ => unreachable!(),
        }
    }
}

struct Field {
    rows: Vec<Vec<Occupancy>>,
    max_y: usize,
    max_x: usize,
}

impl Field {
    fn new(rows: Vec<Vec<Occupancy>>) -> Self {
        let max_y = rows.len() - 1;
        let max_x = rows[0].len() - 1;
        Self { rows, max_y, max_x }
    }

    fn target_count(&self) -> usize {
        let mut groups = HashMap::<_, Vec<Position>>::new();

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == Occupancy::Vacant {
                    continue;
                }

                groups.entry(*cell).or_default().push(Position::new(y, x));
            }
        }

        let mut found = HashSet::new();

        for group in groups.values() {
            for (one, two) in group.iter().tuple_combinations() {
                for target in one.targets_with(two, self.max_y, self.max_x) {
                    found.insert(target);
                }
            }
        }

        found.len()
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut start = None;

    let rows = lines
        .enumerate()
        .map(|(y, line)| {
            line.expect("broken line")
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        start = Some(Position::new(y, x));
                        Occupancy::Vacant
                    } else {
                        Occupancy::from_char(c)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Field::new(rows).target_count()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(14, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(376, solve(std::io::BufReader::new(file).lines()));
}
