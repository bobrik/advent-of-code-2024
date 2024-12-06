use std::io::BufRead;

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
}

#[derive(PartialEq)]
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

        if self.rows[y][x] == Occupancy::Occupied {
            return self.make_a_move(position, direction.rotate());
        }

        let position = Position::new(y, x);

        Some((position, direction))
    }

    fn steps_to_fall_out(&self, start: Position) -> usize {
        let mut visited = self
            .rows
            .iter()
            .map(|row| vec![false; row.len()])
            .collect::<Vec<_>>();

        let mut position = start;
        let mut direction = Direction::North;

        loop {
            visited[position.y][position.x] = true;

            (position, direction) = match self.make_a_move(position, direction) {
                Some((position, direction)) => (position, direction),
                None => break,
            }
        }

        visited
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum()
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

    Field::new(rows).steps_to_fall_out(start.expect("missing starting position"))
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(41, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(5145, solve(std::io::BufReader::new(file).lines()));
}
