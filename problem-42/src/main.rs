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

trait KeyPad {
    fn paths(code: &[char]) -> Vec<Vec<char>> {
        let mut paths = vec![];

        let mut from = 'A';

        for to in code {
            paths.push(Self::single_move(from, *to));
            from = *to;
        }

        paths
    }

    fn single_move(from: char, to: char) -> Vec<char>;
}

struct NumericKeyPad;

impl KeyPad for NumericKeyPad {
    // Some of these that are unused in check.txt and input.txt might be in a sub-optimal order.
    fn single_move(from: char, to: char) -> Vec<char> {
        match from {
            'A' => match to {
                '0' => vec!['<', 'A'],
                '1' => vec!['^', '<', '<', 'A'],
                '2' => vec!['^', '<', 'A'],
                '3' => vec!['^', 'A'],
                '4' => vec!['^', '^', '<', '<', 'A'],
                '5' => vec!['<', '^', '^', 'A'],
                '6' => vec!['^', '^', 'A'],
                '7' => vec!['^', '^', '^', '<', '<', 'A'],
                '8' => vec!['^', '^', '^', '<', 'A'],
                '9' => vec!['^', '^', '^', 'A'],
                _ => unreachable!(),
            },
            '0' => match to {
                'A' => vec!['>', 'A'],
                '1' => vec!['^', '<', 'A'],
                '2' => vec!['^', 'A'],
                '3' => vec!['^', '>', 'A'],
                '4' => vec!['^', '^', '<', 'A'],
                '5' => vec!['^', '^', 'A'],
                '6' => vec!['^', '^', '>', 'A'],
                '7' => vec!['^', '^', '^', '<', 'A'],
                '8' => vec!['^', '^', '^', 'A'],
                '9' => vec!['^', '^', '^', '>', 'A'],
                _ => unreachable!(),
            },
            '1' => match to {
                'A' => vec!['>', '>', 'v', 'A'],
                '0' => vec!['>', 'v', 'A'],
                '2' => vec!['>', 'A'],
                '3' => vec!['>', '>', 'A'],
                '4' => vec!['^', 'A'],
                '5' => vec!['^', '>', 'A'],
                '6' => vec!['^', '>', '>', 'A'],
                '7' => vec!['^', '^', 'A'],
                '8' => vec!['^', '^', '>', 'A'],
                '9' => vec!['^', '^', '>', '>', 'A'],
                _ => unreachable!(),
            },
            '2' => match to {
                'A' => vec!['v', '>', 'A'],
                '0' => vec!['v', 'A'],
                '1' => vec!['<', 'A'],
                '3' => vec!['>', 'A'],
                '4' => vec!['^', '<', 'A'],
                '5' => vec!['^', 'A'],
                '6' => vec!['^', '>', 'A'],
                '7' => vec!['^', '^', '<', 'A'],
                '8' => vec!['^', '^', 'A'],
                '9' => vec!['^', '^', '>', 'A'],
                _ => unreachable!(),
            },
            '3' => match to {
                'A' => vec!['v', 'A'],
                '0' => vec!['v', '<', 'A'],
                '1' => vec!['<', '<', 'A'],
                '2' => vec!['<', 'A'],
                '4' => vec!['^', '<', '<', 'A'],
                '5' => vec!['^', '<', 'A'],
                '6' => vec!['^', 'A'],
                '7' => vec!['<', '<', '^', '^', 'A'],
                '8' => vec!['<', '^', '^', 'A'],
                '9' => vec!['^', '^', 'A'],
                _ => unreachable!(),
            },
            '4' => match to {
                'A' => vec!['>', '>', 'v', 'v', 'A'],
                '0' => vec!['>', 'v', 'v', 'A'],
                '1' => vec!['v', 'A'],
                '2' => vec!['>', 'v', 'A'],
                '3' => vec!['>', '>', 'v', 'A'],
                '5' => vec!['>', 'A'],
                '6' => vec!['>', '>', 'A'],
                '7' => vec!['^', 'A'],
                '8' => vec!['^', '>', 'A'],
                '9' => vec!['^', '>', '>', 'A'],
                _ => unreachable!(),
            },
            '5' => match to {
                'A' => vec!['v', 'v', '>', 'A'],
                '0' => vec!['v', 'v', 'A'],
                '1' => vec!['v', '<', 'A'],
                '2' => vec!['>', 'A'],
                '3' => vec!['v', '>', 'A'],
                '4' => vec!['<', 'A'],
                '6' => vec!['>', 'A'],
                '7' => vec!['^', '<', 'A'],
                '8' => vec!['^', 'A'],
                '9' => vec!['^', '>', 'A'],
                _ => unreachable!(),
            },
            '6' => match to {
                'A' => vec!['v', 'v', 'A'],
                '0' => vec!['v', 'v', '<', 'A'],
                '1' => vec!['<', '<', 'v', 'A'],
                '2' => vec!['<', 'v', 'A'],
                '3' => vec!['v', 'A'],
                '4' => vec!['<', '<', 'A'],
                '5' => vec!['<', 'A'],
                '7' => vec!['<', '<', '^', 'A'],
                '8' => vec!['^', '<', 'A'],
                '9' => vec!['^', 'A'],
                _ => unreachable!(),
            },
            '7' => match to {
                'A' => vec!['>', '>', 'v', 'v', 'v', 'A'],
                '0' => vec!['>', 'v', 'v', 'v', 'A'],
                '1' => vec!['v', 'v', 'A'],
                '2' => vec!['v', 'v', '>', 'A'],
                '3' => vec!['v', 'v', '>', '>', 'A'],
                '4' => vec!['v', 'A'],
                '5' => vec!['v', '>', 'A'],
                '6' => vec!['v', '>', '>', 'A'],
                '8' => vec!['>', 'A'],
                '9' => vec!['>', '>', 'A'],
                _ => unreachable!(),
            },
            '8' => match to {
                'A' => vec!['>', 'v', 'v', 'v', 'A'],
                '0' => vec!['v', 'v', 'v', 'A'],
                '1' => vec!['v', 'v', '<', 'A'],
                '2' => vec!['v', 'v', 'A'],
                '3' => vec!['v', 'v', '>', 'A'],
                '4' => vec!['v', '<', 'A'],
                '5' => vec!['v', 'A'],
                '6' => vec!['v', '>', 'A'],
                '7' => vec!['<', 'A'],
                '9' => vec!['>', 'A'],
                _ => unreachable!(),
            },
            '9' => match to {
                '0' => vec!['v', 'v', 'v', '<', 'A'],
                'A' => vec!['v', 'v', 'v', 'A'],
                '1' => vec!['v', 'v', '<', '<', 'A'],
                '2' => vec!['v', 'v', '<', 'A'],
                '3' => vec!['v', 'v', 'A'],
                '4' => vec!['v', '<', '<', 'A'],
                '5' => vec!['v', '<', 'A'],
                '6' => vec!['v', 'A'],
                '7' => vec!['<', '<', 'A'],
                '8' => vec!['<', 'A'],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

struct ArrowPad;

impl KeyPad for ArrowPad {
    fn single_move(from: char, to: char) -> Vec<char> {
        match from {
            'A' => match to {
                'A' => vec!['A'],
                '^' => vec!['<', 'A'],
                '>' => vec!['v', 'A'],
                'v' => vec!['<', 'v', 'A'],
                '<' => vec!['v', '<', '<', 'A'],
                _ => unreachable!(),
            },
            '^' => match to {
                'A' => vec!['>', 'A'],
                '^' => vec!['A'],
                '>' => vec!['v', '>', 'A'],
                'v' => vec!['v', 'A'],
                '<' => vec!['v', '<', 'A'],
                _ => unreachable!(),
            },
            'v' => match to {
                'A' => vec!['^', '>', 'A'],
                '^' => vec!['^', 'A'],
                '>' => vec!['>', 'A'],
                'v' => vec!['A'],
                '<' => vec!['<', 'A'],
                _ => unreachable!(),
            },
            '<' => match to {
                'A' => vec!['>', '>', '^', 'A'],
                '^' => vec!['>', '^', 'A'],
                '>' => vec!['>', '>', 'A'],
                'v' => vec!['>', 'A'],
                '<' => vec!['A'],
                _ => unreachable!(),
            },
            '>' => match to {
                'A' => vec!['^', 'A'],
                '^' => vec!['<', '^', 'A'],
                '>' => vec!['A'],
                '<' => vec!['<', '<', 'A'],
                'v' => vec!['<', 'A'],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn sequence_len(code: &[char], layers: usize) -> usize {
    let first = NumericKeyPad::paths(code)
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let mut cache = FxHashMap::default();

    let mut len = 0;

    let mut from = 'A';

    for to_idx in 0..first.len() {
        len += single_hop_len(from, first[to_idx], &mut cache, layers);
        from = first[to_idx];
    }

    eprintln!("{code:?} -> {len}");

    len
}

fn single_hop_len(
    prev: char,
    next: char,
    cache: &mut FxHashMap<(char, char, usize), usize>,
    level: usize,
) -> usize {
    if level == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(prev, next, level)) {
        return *result;
    }

    let mut len = 0;

    let mut from = 'A';

    for to in ArrowPad::single_move(prev, next) {
        len += single_hop_len(from, to, cache, level - 1);
        from = to;
    }

    cache.insert((prev, next, level), len);

    len
}

fn numeric(code: &[char]) -> usize {
    code[..3]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .expect("error parsing code as number")
}

fn solve<T: BufRead>(lines: std::io::Lines<T>) -> usize {
    let codes = lines
        .into_iter()
        .map(|line| line.expect("broken line").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    codes
        .iter()
        .map(|code| sequence_len(code, 25) * numeric(code))
        .sum()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(
        154115708116294,
        solve(std::io::BufReader::new(file).lines())
    );

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(
        263492840501566,
        solve(std::io::BufReader::new(file).lines())
    );
}