use std::io::BufRead;

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

struct Field {
    rows: Vec<Vec<u8>>,
    max_y: usize,
    max_x: usize,
}

impl Field {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        let max_y = rows.len() - 1;
        let max_x = rows[0].len() - 1;
        Self { rows, max_y, max_x }
    }

    fn start_positions(&self) -> Vec<Position> {
        (0..=self.max_y)
            .flat_map(|y| {
                (0..=self.max_x).filter_map(move |x| {
                    if self.rows[y][x] == 0 {
                        Some(Position::new(y, x))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn rating_from(&self, start: Position) -> usize {
        let mut rating = 0;

        let mut stack = vec![(start)];

        while let Some(position) = stack.pop() {
            let value = self.rows[position.y][position.x];

            if value == 9 {
                rating += 1;
                continue;
            }

            for direction in DIRECTIONS {
                if let Some(next) = position.next(*direction, self.max_y, self.max_x) {
                    if self.rows[next.y][next.x] as i8 - value as i8 != 1 {
                        continue;
                    }

                    stack.push(next);
                }
            }
        }

        rating
    }

    fn rating(&self) -> usize {
        self.start_positions()
            .into_iter()
            .map(|start| self.rating_from(start))
            .sum()
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let rows = lines
        .map(|line| {
            line.expect("broken line")
                .chars()
                .map(|c| c.to_digit(10).expect("error parsing cell") as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Field::new(rows).rating()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(81, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1960, solve(std::io::BufReader::new(file).lines()));
}
