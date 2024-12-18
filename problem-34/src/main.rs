use std::io::BufRead;

use itertools::Itertools;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let started = std::time::Instant::now();
    let solution = solve(lines);
    let elapsed = started.elapsed();

    println!("Solution: {} [{}us]", solution, elapsed.as_micros())
}

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn from_u8(opcode: u8) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Opcode,
    operand: u8,
}

impl Instruction {
    fn new(opcode: Opcode, operand: u8) -> Self {
        Self { opcode, operand }
    }

    fn combo(&self, register_a: &usize, register_b: &usize, register_c: &usize) -> usize {
        match self.operand {
            0..=3 => self.operand as usize,
            4 => *register_a,
            5 => *register_b,
            6 => *register_c,
            _ => unreachable!(),
        }
    }

    #[allow(clippy::assign_op_pattern)]
    fn apply(
        &self,
        register_a: &mut usize,
        register_b: &mut usize,
        register_c: &mut usize,
        instruction_pointer: &mut usize,
    ) -> Option<u8> {
        let mut output = None;

        match self.opcode {
            Opcode::Adv => {
                *register_a =
                    *register_a / 2usize.pow(self.combo(register_a, register_b, register_c) as u32)
            }
            Opcode::Bxl => *register_b = *register_b ^ self.operand as usize,
            Opcode::Bst => *register_b = self.combo(register_a, register_b, register_c) % 8,
            Opcode::Jnz => {
                if *register_a != 0 {
                    *instruction_pointer = self.operand as usize;
                    return None;
                }
            }
            Opcode::Bxc => *register_b = *register_b ^ *register_c,
            Opcode::Out => {
                output = Some((self.combo(register_a, register_b, register_c) % 8) as u8)
            }
            Opcode::Bdv => {
                *register_b =
                    *register_a / 2usize.pow(self.combo(register_a, register_b, register_c) as u32)
            }
            Opcode::Cdv => {
                *register_c =
                    *register_a / 2usize.pow(self.combo(register_a, register_b, register_c) as u32)
            }
        }

        *instruction_pointer += 1;

        output
    }
}

#[derive(Clone, Debug)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instruction_pointer: usize,
    program: Vec<Instruction>,
}

impl Computer {
    fn new(
        register_a: usize,
        register_b: usize,
        register_c: usize,
        program: Vec<Instruction>,
    ) -> Self {
        let instruction_pointer = 0;

        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer,
            program,
        }
    }

    fn find_lsb(&mut self, register_a: usize, expected: &[u8]) -> Option<usize> {
        self.register_a = register_a;
        self.register_b = 0;
        self.register_c = 0;

        let mut check_idx = 0;

        self.instruction_pointer = 0;

        loop {
            if self.instruction_pointer >= self.program.len() {
                break;
            }

            if let Some(output) = self.program[self.instruction_pointer].apply(
                &mut self.register_a,
                &mut self.register_b,
                &mut self.register_c,
                &mut self.instruction_pointer,
            ) {
                if output != expected[check_idx] {
                    return None;
                }

                if check_idx == expected.len() - 1 {
                    break;
                }

                check_idx += 1;
            }
        }

        if check_idx == expected.len() - 1 {
            return Some(register_a & ((1 << (expected.len() * 3)) - 1));
        }

        None
    }
}

fn solve<T: BufRead>(mut lines: std::io::Lines<T>) -> usize {
    let mut register_a = None;
    let mut register_b = None;
    let mut register_c = None;

    for line in lines.by_ref() {
        let line = line.expect("broken line");
        if line.is_empty() {
            break;
        }

        if let Some(value) = line.strip_prefix("Register A: ") {
            register_a = Some(value.parse().expect("error parsing register a"));
        } else if let Some(value) = line.strip_prefix("Register B: ") {
            register_b = Some(value.parse().expect("error parsing register b"));
        } else if let Some(value) = line.strip_prefix("Register C: ") {
            register_c = Some(value.parse().expect("error parsing register c"));
        } else {
            eprintln!("line = {line}");
            unreachable!();
        }
    }

    // The code from input.txt:
    //
    // do {
    //     [0] b = b % 8
    //     [1] b = b ^ 3
    //     [2] b = a / (1 << b)
    //     [3] b = b ^ c
    //     [4] b = b ^ 3
    //     [5] a = a / (1 << 3)
    //     [6] out b % 3
    // } while (a != 0)
    //
    // Observations:
    // * A is divided by 8 (3 bit shift) each iteration
    // * A is between 2^45 and 2^47 (3 bits per each byte of code)
    // * B is carried over between iterations
    // * B is truncated at 3 lowest bits during output, but not between iterations
    // * C does not matter at all and it does not change
    // * Each iteration takes 3 bits of A + 3 bits of B and produces an output and the new value of A and B
    // * Common outputs have a common prefix of least significant bits of A (except for the some top ones)

    let code = lines
        .next()
        .expect("missing program line")
        .expect("broken program line")
        .strip_prefix("Program: ")
        .expect("missing program prefix")
        .split(',')
        .map(|n| n.parse::<u8>().expect("error parsing program code"))
        .collect::<Vec<_>>();

    let program = code
        .iter()
        .tuples()
        .map(|(opcode, operand)| {
            let opcode = Opcode::from_u8(*opcode);
            Instruction::new(opcode, *operand)
        })
        .collect::<Vec<_>>();

    let register_a = register_a.expect("missing register a");
    let register_b = register_b.expect("missing register b");
    let register_c = register_c.expect("missing register c");

    let mut computer = Computer::new(register_a, register_b, register_c, program);

    let mut discovered_bits = 0;

    // Discover the least significant bits of A by looking for 4 first numbers in the output,
    // then try extending the value by checking 2 addition numbers at a time. This is not
    // really guaranteed to work, but it does and I'm tired.
    for cap in (4..=code.len()).step_by(2) {
        for candidate_extension in 0.. {
            let candidate = (candidate_extension << ((cap - 4) * 3)) + discovered_bits;
            if let Some(lsb) = computer.find_lsb(candidate, &code[0..cap]) {
                if cap == code.len() {
                    return candidate;
                }

                discovered_bits = lsb;
                break;
            }
        }
    }

    unreachable!()
}

#[test]
fn test_solution() {
    let file = std::fs::File::open("check.txt").expect("cannot open input");
    assert_eq!(117440, solve(std::io::BufReader::new(file).lines()));

    let file = std::fs::File::open("input.txt").expect("cannot open input");
    assert_eq!(
        108107574778365,
        solve(std::io::BufReader::new(file).lines())
    );
}
