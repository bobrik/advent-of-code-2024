use std::io::BufRead;

use fxhash::FxHashMap;

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
    let mut two = FxHashMap::default();

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

        *two.entry(
            parts
                .next()
                .expect("missing right column")
                .parse::<usize>()
                .expect("error parsing number"),
        )
        .or_default() += 1;
    }

    one.iter()
        .map(|left| left * two.get(left).unwrap_or(&0))
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(31, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(23387399, solve(std::io::BufReader::new(file).lines()));
}
