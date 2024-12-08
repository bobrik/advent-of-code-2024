use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

enum Map {
    File(usize, u8),
    Free(u8),
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    let mut maps = lines
        .next()
        .expect("missing line")
        .expect("broken line")
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            let size = c.to_digit(10).expect("error parsing digit") as u8;

            if idx % 2 == 0 {
                Map::File(idx / 2, size)
            } else {
                Map::Free(size)
            }
        })
        .collect::<Vec<_>>();

    let mut skip_forward = 0;
    let mut skip_backward = 0;

    loop {
        let candidate = maps
            .iter()
            .enumerate()
            .rev()
            .skip(skip_backward)
            .filter_map(|(idx, map)| match map {
                Map::File(id, size) => Some((idx, *id, *size)),
                Map::Free(_) => None,
            })
            .next();

        let Some((candidate_map_idx, candidate_id, candidate_size)) = candidate else {
            break;
        };

        let mut insert = None;
        let mut filled = true;

        for (map_idx, map) in maps.iter().enumerate().skip(skip_forward) {
            if let Map::Free(free_size) = map {
                if *free_size != 0 {
                    filled = false;
                }

                if filled {
                    skip_forward = map_idx;
                }

                if map_idx >= candidate_map_idx {
                    break;
                }

                if candidate_size <= *free_size {
                    insert = Some((
                        candidate_map_idx,
                        candidate_id,
                        candidate_size,
                        map_idx,
                        *free_size,
                    ));

                    break;
                }
            }
        }

        if let Some((candidate_map_idx, candidate_id, candidate_size, free_map_idx, free_size)) =
            insert
        {
            maps[candidate_map_idx] = Map::Free(candidate_size);

            if free_size == candidate_size {
                maps[free_map_idx] = Map::File(candidate_id, candidate_size);
            } else {
                maps[free_map_idx] = Map::Free(free_size - candidate_size);
                maps.insert(free_map_idx, Map::File(candidate_id, candidate_size));
            }
        }

        skip_backward = maps.len() - candidate_map_idx;
    }

    let mut checksum = 0;
    let mut block_idx = 0;

    for map in maps {
        match map {
            Map::File(id, size) => {
                for _ in 0..size {
                    checksum += block_idx * id;
                    block_idx += 1;
                }
            }
            Map::Free(size) => block_idx += size as usize,
        };
    }

    checksum
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(2858, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(6493634986625, solve(std::io::BufReader::new(file).lines()));
}
