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

    fn min_time(
        &self,
        start: Position,
        end: Position,
        occupied: &FxHashSet<Position>,
        cheat: Option<Position>,
    ) -> Option<usize> {
        let mut seen = FxHashSet::default();
        seen.insert(start);

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((position, time)) = queue.pop_front() {
            if position == end {
                return Some(time);
            }

            for direction in DIRECTIONS {
                let Some(candidate) = position.next(*direction, self.max_y, self.max_x) else {
                    continue;
                };

                if occupied.contains(&candidate) {
                    if let Some(cheat) = cheat {
                        if cheat != candidate {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                if seen.insert(candidate) {
                    queue.push_back((candidate, time + 1));
                }
            }
        }

        None
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut start = None;
    let mut end = None;

    let mut max_y = 0;
    let mut max_x = 0;

    let occupied = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.expect("broken line")
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    max_y = max_y.max(y);
                    max_x = max_x.max(x);

                    let position = Position::new(y, x);

                    match c {
                        'S' => {
                            start = Some(position);
                            None
                        }
                        'E' => {
                            end = Some(position);
                            None
                        }
                        '#' => Some(position),
                        '.' => None,
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<FxHashSet<_>>();

    let start = start.expect("missing start position");
    let end = end.expect("missing start position");

    let field = Field::new(max_y, max_x);

    let min_time = field
        .min_time(start, end, &occupied, None)
        .expect("no path");

    let diff = if max_x > 100 { 100 } else { 1 };

    occupied
        .iter()
        .filter_map(|cheat| field.min_time(start, end, &occupied, Some(*cheat)))
        .filter(|time| *time + diff <= min_time)
        .count()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(44, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1395, solve(std::io::BufReader::new(file).lines()));
}
