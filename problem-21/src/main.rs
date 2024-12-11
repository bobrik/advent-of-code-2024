use std::io::BufRead;

use rustc_hash::FxHashMap;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

fn compute_number_split(
    number: usize,
    steps: usize,
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if steps == 0 {
        return 1;
    }

    match cache.get(&(number, steps)) {
        Some(value) => *value,
        None => {
            let value = if number == 0 {
                compute_number_split(1, steps - 1, cache)
            } else {
                let digits = number.ilog10() + 1;

                if digits % 2 == 0 {
                    let mid = 10usize.pow(digits / 2);

                    compute_number_split(number / mid, steps - 1, cache)
                        + compute_number_split(number % mid, steps - 1, cache)
                } else {
                    compute_number_split(number * 2024, steps - 1, cache)
                }
            };

            cache.insert((number, steps), value);

            value
        }
    }
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    let numbers = lines
        .next()
        .expect("missing line")
        .expect("broken line")
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().expect("error parsing number"))
        .collect::<Vec<_>>();

    let mut cache = FxHashMap::default();

    numbers
        .into_iter()
        .map(|number| compute_number_split(number, 25, &mut cache))
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(55312, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(224529, solve(std::io::BufReader::new(file).lines()));
}
