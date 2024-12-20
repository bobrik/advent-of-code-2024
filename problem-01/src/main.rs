use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let mut one = vec![];
    let mut two = vec![];

    for line in lines {
        let line = line.expect("broken line");

        let mut parts = line.split_ascii_whitespace();

        one.push(
            parts
                .next()
                .expect("missing left column")
                .parse::<usize>()
                .expect("error parsing number"),
        );

        two.push(
            parts
                .next()
                .expect("missing right column")
                .parse::<usize>()
                .expect("error parsing number"),
        );
    }

    one.sort_unstable();
    two.sort_unstable();

    one.iter()
        .enumerate()
        .map(|(i, left)| left.abs_diff(two[i]))
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(11, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(1197984, solve(std::io::BufReader::new(file).lines()));
}
