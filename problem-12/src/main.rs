use std::io::BufRead;

use bitvec::{bitvec, vec::BitVec};

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

#[derive(Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

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

    fn rotate(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn index(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    fn variant_count() -> usize {
        4
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Occupancy {
    Vacant,
    Occupied,
}

impl Occupancy {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Vacant,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

struct Seen {
    inner: BitVec,
    max_x: usize,
}

impl Seen {
    fn new(max_y: usize, max_x: usize) -> Self {
        let inner = bitvec![0; max_y * max_x * Direction::variant_count()];

        Self { inner, max_x }
    }

    fn index(&self, position: Position, direction: Direction) -> usize {
        position.y * (self.max_x * Direction::variant_count())
            + position.x * Direction::variant_count()
            + direction.index()
    }

    fn insert(&mut self, position: Position, direction: Direction) -> bool {
        let index = self.index(position, direction);

        !self.inner.replace(index, true)
    }

    fn zero(&mut self) {
        self.inner.fill(false);
    }
}

struct Field {
    rows: Vec<Vec<Occupancy>>,
}

impl Field {
    fn new(rows: Vec<Vec<Occupancy>>) -> Self {
        Self { rows }
    }

    fn make_a_move(
        &self,
        position: Position,
        direction: Direction,
        extra_obstacle: Option<Position>,
    ) -> Option<(Position, Direction)> {
        let (dy, dx) = direction.diff();

        let y = position.y as isize + dy;
        let x = position.x as isize + dx;

        if y < 0 || y as usize > self.rows.len() - 1 {
            return None;
        }

        if x < 0 || x as usize > self.rows[y as usize].len() - 1 {
            return None;
        }

        let y = y as usize;
        let x = x as usize;

        let occupancy = match extra_obstacle {
            Some(extra_obstacle) => {
                if y == extra_obstacle.y && x == extra_obstacle.x {
                    Occupancy::Occupied
                } else {
                    self.rows[y][x]
                }
            }
            None => self.rows[y][x],
        };

        if occupancy == Occupancy::Occupied {
            return self.make_a_move(position, direction.rotate(), extra_obstacle);
        }

        let position = Position::new(y, x);

        Some((position, direction))
    }

    fn is_loop_with_obstacle_in(
        &self,
        obstacle: Position,
        start: Position,
        seen: &mut Seen,
    ) -> bool {
        let mut position = start;
        let mut direction = Direction::North;

        loop {
            if !seen.insert(position, direction) {
                return true;
            }

            (position, direction) = match self.make_a_move(position, direction, Some(obstacle)) {
                Some((position, direction)) => (position, direction),
                None => break,
            };
        }

        false
    }

    fn trace_path(&self, start: Position) -> Vec<Position> {
        let mut visited = self
            .rows
            .iter()
            .map(|row| vec![false; row.len()])
            .collect::<Vec<_>>();

        let mut position = start;
        let mut direction = Direction::North;

        loop {
            visited[position.y][position.x] = true;

            (position, direction) = match self.make_a_move(position, direction, None) {
                Some((position, direction)) => (position, direction),
                None => break,
            }
        }

        visited
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, visited)| {
                    if *visited {
                        Some(Position::new(y, x))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn count_possible_obstacles(&self, start: Position) -> usize {
        let mut seen = Seen::new(self.rows.len(), self.rows[0].len());

        self.trace_path(start)
            .into_iter()
            .filter(|candidate| {
                seen.zero();

                self.is_loop_with_obstacle_in(*candidate, start, &mut seen)
            })
            .count()
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut position = None;

    let rows = lines
        .enumerate()
        .map(|(y, line)| {
            line.expect("broken line")
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        position = Some(Position::new(y, x));
                        Occupancy::Vacant
                    } else {
                        Occupancy::from_char(c)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Field::new(rows).count_possible_obstacles(position.expect("missing starting position"))
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(6, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1523, solve(std::io::BufReader::new(file).lines()));
}
