use std::{cmp::Ordering, collections::BinaryHeap, io::BufRead};

use rustc_hash::FxHashSet;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

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

    fn rotate_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
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

    fn next(&self, direction: Direction, max_y: usize, max_x: usize) -> Position {
        let (dy, dx) = direction.diff();

        self.next_diff(dy, dx, max_y, max_x)
            .expect("cannot run off map")
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Occupancy {
    Empty,
    Wall,
}

impl Occupancy {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq)]
struct State {
    position: Position,
    direction: Direction,
    score: usize,
}

impl State {
    fn new(position: Position, direction: Direction, score: usize) -> Self {
        Self {
            position,
            direction,
            score,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
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

    fn min_score(&self, start: Position, end: Position) -> usize {
        let mut seen = FxHashSet::default();
        seen.insert(start);

        let mut queue = BinaryHeap::new();
        queue.push(State::new(start, Direction::East, 0));

        while let Some(state) = queue.pop() {
            if state.position == end {
                return state.score;
            }

            for (direction, score_diff) in [
                (state.direction, 1),
                (state.direction.rotate_clockwise(), 1001),
                (state.direction.rotate_counter_clockwise(), 1001),
            ] {
                let candidate = state.position.next(direction, self.max_y, self.max_x);
                if self.rows[candidate.y][candidate.x] == Occupancy::Empty && seen.insert(candidate)
                {
                    queue.push(State::new(candidate, direction, state.score + score_diff));
                }
            }
        }

        0
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut start = None;
    let mut end = None;

    let rows = lines
        .enumerate()
        .map(|(y, line)| {
            line.expect("broken line")
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some(Position::new(y, x));
                        Occupancy::Empty
                    } else if c == 'E' {
                        end = Some(Position::new(y, x));
                        Occupancy::Empty
                    } else {
                        Occupancy::from_char(c)
                    }
                })
                .collect()
        })
        .collect();

    let start = start.expect("missing start position");
    let end = end.expect("missing start position");

    let field = Field::new(rows);

    field.min_score(start, end)
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(7036, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(85480, solve(std::io::BufReader::new(file).lines()));
}
