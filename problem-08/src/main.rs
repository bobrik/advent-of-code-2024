use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
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

    fn is_xmas_point(&self, y: usize, x: usize) -> bool {
        if self.rows[y][x] != 'A' {
            return false;
        }

        // M.S
        // .A.
        // M.S
        if self.is_desired(y, x, -1, 1, 'S')
            && self.is_desired(y, x, 1, 1, 'S')
            && self.is_desired(y, x, 1, -1, 'M')
            && self.is_desired(y, x, -1, -1, 'M')
        {
            return true;
        }

        // S.M
        // .A.
        // S.M
        if self.is_desired(y, x, -1, 1, 'M')
            && self.is_desired(y, x, 1, 1, 'M')
            && self.is_desired(y, x, 1, -1, 'S')
            && self.is_desired(y, x, -1, -1, 'S')
        {
            return true;
        }

        // S.S
        // .A.
        // M.M
        if self.is_desired(y, x, -1, 1, 'S')
            && self.is_desired(y, x, 1, 1, 'M')
            && self.is_desired(y, x, 1, -1, 'M')
            && self.is_desired(y, x, -1, -1, 'S')
        {
            return true;
        }

        // M.M
        // .A.
        // S.S
        if self.is_desired(y, x, -1, 1, 'M')
            && self.is_desired(y, x, 1, 1, 'S')
            && self.is_desired(y, x, 1, -1, 'S')
            && self.is_desired(y, x, -1, -1, 'M')
        {
            return true;
        }

        false
    }

    fn count_xmas_points(&self) -> usize {
        let mut count = 0;
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                if self.is_xmas_point(y, x) {
                    count += 1;
                }
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
    assert_eq!(9, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1921, solve(std::io::BufReader::new(file).lines()));
}
