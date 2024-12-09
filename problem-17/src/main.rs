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

    let mut checksum = 0;

    let mut map_idx = 0;
    let mut block_idx = 0;

    loop {
        if map_idx >= maps.len() {
            break;
        }

        match maps[map_idx] {
            Map::File(id, size) => {
                for _ in 0..size {
                    checksum += block_idx * id;
                    block_idx += 1;
                }
            }
            Map::Free(mut free_size) => loop {
                if free_size == 0 {
                    break;
                }

                if map_idx == maps.len() - 1 {
                    break;
                }

                if let Map::File(id, size) = maps.pop().expect("missing map") {
                    let drain = free_size.min(size);

                    for _ in 0..drain {
                        checksum += block_idx * id;
                        block_idx += 1;
                    }

                    free_size -= drain;

                    if drain < size {
                        maps.push(Map::File(id, size - drain));
                    }
                }
            },
        }

        map_idx += 1;
    }

    checksum
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(1928, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(6463499258318, solve(std::io::BufReader::new(file).lines()));
}
