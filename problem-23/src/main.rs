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

#[derive(Clone, Copy)]
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

struct Region {
    inner: FxHashSet<Position>,
}

impl Region {
    fn new_from(field: &Field, start: Position) -> Self {
        let mut inner = FxHashSet::default();
        inner.insert(start);

        let letter = field.rows[start.y][start.x];

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(position) = queue.pop_front() {
            for direction in DIRECTIONS {
                if let Some(next) = position.next(*direction, field.max_y, field.max_x) {
                    if field.rows[next.y][next.x] != letter {
                        continue;
                    }

                    if !inner.insert(next) {
                        continue;
                    }

                    queue.push_back(next);
                }
            }
        }

        Self { inner }
    }

    fn contains(&self, position: Position) -> bool {
        self.inner.contains(&position)
    }

    fn perimeter(&self, field: &Field) -> usize {
        self.inner
            .iter()
            .map(|position| {
                DIRECTIONS
                    .iter()
                    .filter(|direction| self.is_perimeter(field, *position, **direction))
                    .count()
            })
            .sum()
    }

    fn is_perimeter(&self, field: &Field, position: Position, direction: Direction) -> bool {
        match position.next(direction, field.max_y, field.max_x) {
            Some(next) => !self.contains(next),
            None => true,
        }
    }

    fn area(&self) -> usize {
        self.inner.len()
    }

    fn price(&self, field: &Field) -> usize {
        self.perimeter(field) * self.area()
    }
}

struct Field {
    rows: Vec<Vec<char>>,
    max_y: usize,
    max_x: usize,
}

impl Field {
    fn new(rows: Vec<Vec<char>>) -> Self {
        let max_y = rows.len() - 1;
        let max_x = rows[0].len() - 1;

        Self { rows, max_y, max_x }
    }

    fn regions(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = vec![];

        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let candidate = Position::new(y, x);

                if regions.iter().any(|region| region.contains(candidate)) {
                    continue;
                }

                regions.push(Region::new_from(self, candidate));
            }
        }

        regions
    }

    fn fence_cost(&self) -> usize {
        self.regions()
            .into_iter()
            .map(|region| region.price(self))
            .sum()
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let rows = lines
        .map(|line| line.expect("broken line").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Field::new(rows).fence_cost()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(1930, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1363682, solve(std::io::BufReader::new(file).lines()));
}
