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
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn diff(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        }
    }
}

struct Field {
    rows: Vec<Vec<char>>,
}

impl Field {
    fn new(rows: Vec<Vec<char>>) -> Self {
        Self { rows }
    }

    fn value_at(&self, y: usize, x: usize, dy: isize, dx: isize) -> Option<char> {
        let y = y as isize + dy;
        let x = x as isize + dx;

        if y as usize > self.rows.len() - 1 || y < 0 {
            return None;
        }

        if x as usize > self.rows[y as usize].len() - 1 || x < 0 {
            return None;
        }

        Some(self.rows[y as usize][x as usize])
    }

    fn is_desired(&self, y: usize, x: usize, dy: isize, dx: isize, desired: char) -> bool {
        self.value_at(y, x, dy, dx) == Some(desired)
    }

    fn is_xmas_in_direction(&self, mut y: usize, mut x: usize, dy: isize, dx: isize) -> bool {
        if self.rows[y][x] != 'X' {
            return false;
        }

        let mut desired = 'M';

        loop {
            if !self.is_desired(y, x, dy, dx, desired) {
                return false;
            }

            y = (y as isize + dy) as usize;
            x = (x as isize + dx) as usize;

            desired = match desired {
                'M' => 'A',
                'A' => 'S',
                'S' => return true,
                _ => unreachable!(),
            }
        }
    }

    fn xmas_point_directions(&self, y: usize, x: usize) -> usize {
        DIRECTIONS
            .iter()
            .filter(|direction| {
                let (dy, dx) = direction.diff();

                self.is_xmas_in_direction(y, x, dy, dx)
            })
            .count()
    }

    fn count_xmas_points(&self) -> usize {
        let mut count = 0;
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                count += self.xmas_point_directions(y, x);
            }
        }

        count
    }
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let rows = lines
        .map(|line| line.expect("broken line").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Field::new(rows).count_xmas_points()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(18, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(2530, solve(std::io::BufReader::new(file).lines()));
}
