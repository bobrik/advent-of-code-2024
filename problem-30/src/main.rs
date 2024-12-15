use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }

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
    BoxLeft,
    BoxRight,
}

impl Occupancy {
    fn from_char(c: char) -> Vec<Self> {
        match c {
            '.' => vec![Self::Empty, Self::Empty],
            '#' => vec![Self::Wall, Self::Wall],
            'O' => vec![Self::BoxLeft, Self::BoxRight],
            _ => unreachable!(),
        }
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

    fn can_move_from(&self, from: Position, direction: Direction) -> bool {
        if direction == Direction::East || direction == Direction::West {
            return self.can_move_single(from, direction);
        }

        match self.rows[from.y][from.x] {
            Occupancy::BoxLeft => {
                let from_right = from.next(Direction::East, self.max_y, self.max_x);

                self.can_move_single(from, direction) && self.can_move_single(from_right, direction)
            }
            Occupancy::BoxRight => {
                let from_left = from.next(Direction::West, self.max_y, self.max_x);

                self.can_move_single(from, direction) && self.can_move_single(from_left, direction)
            }
            _ => self.can_move_single(from, direction),
        }
    }

    fn can_move_single(&self, from: Position, direction: Direction) -> bool {
        let next = from.next(direction, self.max_y, self.max_x);

        match self.rows[next.y][next.x] {
            Occupancy::Empty => true,
            Occupancy::Wall => false,
            Occupancy::BoxLeft | Occupancy::BoxRight => self.can_move_from(next, direction),
        }
    }

    fn move_from(&mut self, from: Position, direction: Direction) {
        if direction == Direction::East || direction == Direction::West {
            return self.move_single(from, direction);
        }

        match self.rows[from.y][from.x] {
            Occupancy::BoxLeft => {
                let from_right = from.next(Direction::East, self.max_y, self.max_x);

                self.move_single(from, direction);
                self.move_single(from_right, direction);
            }
            Occupancy::BoxRight => {
                let from_left = from.next(Direction::West, self.max_y, self.max_x);

                self.move_single(from, direction);
                self.move_single(from_left, direction);
            }
            _ => unreachable!(),
        }
    }

    fn move_single(&mut self, from: Position, direction: Direction) {
        let next = from.next(direction, self.max_y, self.max_x);

        match self.rows[next.y][next.x] {
            Occupancy::Empty => {
                self.rows[next.y][next.x] = self.rows[from.y][from.x];
                self.rows[from.y][from.x] = Occupancy::Empty;
            }
            Occupancy::Wall => unreachable!(),
            Occupancy::BoxLeft | Occupancy::BoxRight => {
                self.move_from(next, direction);
                self.rows[next.y][next.x] = self.rows[from.y][from.x];
                self.rows[from.y][from.x] = Occupancy::Empty;
            }
        }
    }

    fn apply(&mut self, mut robot: Position, moves: &[Direction]) {
        for direction in moves {
            let next = robot.next(*direction, self.max_y, self.max_x);

            match self.rows[next.y][next.x] {
                Occupancy::Empty => {
                    robot = next;
                }
                Occupancy::Wall => (),
                Occupancy::BoxLeft | Occupancy::BoxRight => {
                    if self.can_move_from(robot, *direction) {
                        self.move_single(robot, *direction);
                        robot = next;
                    }
                }
            }
        }
    }

    fn sum_of_box_coordinates(&self) -> usize {
        (0..=self.max_y)
            .flat_map(|y| {
                (0..=self.max_x).map(move |x| {
                    if self.rows[y][x] == Occupancy::BoxLeft {
                        100 * y + x
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    let mut rows = vec![];
    let mut robot = None;

    for (y, line) in lines.by_ref().enumerate() {
        let line = line.expect("broken line");
        if line.is_empty() {
            break;
        }

        rows.push(
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| {
                    if c == '@' {
                        robot = Some(Position::new(y, x * 2));
                        Occupancy::from_char('.')
                    } else {
                        Occupancy::from_char(c)
                    }
                })
                .collect(),
        );
    }

    let robot = robot.expect("missing robot position");

    let moves = lines
        .next()
        .expect("missing move line")
        .expect("broke moves line")
        .chars()
        .map(Direction::from_char)
        .collect::<Vec<_>>();

    let mut field = Field::new(rows);

    field.apply(robot, &moves);

    field.sum_of_box_coordinates()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(9021, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1392847, solve(std::io::BufReader::new(file).lines()));
}
