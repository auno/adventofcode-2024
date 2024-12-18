use std::ops::{Index, IndexMut};

use anyhow::{bail, ensure, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::derive::TryFrom;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFrom)]
#[try_from(repr)]
#[repr(usize)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

type Registers = [usize; 3];

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFrom)]
#[try_from(repr)]
#[repr(usize)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
}

impl Index<Register> for Registers {
    type Output = usize;

    fn index(&self, register: Register) -> &Self::Output {
        &self[register as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self[register as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Operand {
    Literal(usize),
    Register(Register),
}

impl TryFrom<(Instruction, usize)> for Operand {
    type Error = Error;

    fn try_from(value: (Instruction, usize)) -> Result<Self> {
        use Instruction::*;
        use Operand::*;

        let operand = match value {
            (Bxl | Jnz | Bxc, v) => Literal(v),
            (Adv | Bdv | Cdv | Bst | Out, v @ 0..=3) => Literal(v),
            (Adv | Bdv | Cdv | Bst | Out, v @ 4..=6) => Register((v - 4).try_into()?),
            (Adv | Bdv | Cdv | Bst | Out, 7) => bail!("Invalid combo operand: 7"),
            _ => bail!("Invalid instruction/operand combination: {value:?}")
        };

        Ok(operand)
    }
}

impl Operand {
    fn resolve_value(self, registers: &Registers) -> usize {
        match self {
            Operand::Literal(value) => value,
            Operand::Register(register) => registers[register],
        }
    }
}

type Input = (Registers, Vec<usize>);

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<Input> {
    let (registers, program) = input.split_once("\n\n").context("Unable to parse input")?;

    let (a, b, c) = scan_fmt!(registers, "Register A: {d}\nRegister B: {d}\nRegister C: {d}", usize, usize, usize)?;
    let registers = [a, b, c];

    let (_, program) = program.split_once(' ').context("Unable to parse program")?;
    let program = program
        .trim()
        .split(',')
        .map(|num| num.parse().context(format!("Unable to parse number: {num}")))
        .collect::<Result<_>>()?;

    Ok((registers, program))
}

fn parse_operation(program: &[usize]) -> Result<(Instruction, Operand)> {
    ensure!(program.len() >= 2, "Unable to parse operation, program segment to short: {program:?}");

    let instruction = program[0].try_into()?;
    let operand = (instruction, program[1]).try_into()?;

    Ok((instruction, operand))
}

fn run_program(program: &[usize], registers: Registers) -> Result<(Vec<usize>, Registers)> {
    let mut registers = registers;
    let mut ip = 0;
    let mut output: Vec<usize> = vec![];

    loop {
        if ip >= program.len() {
            break;
        }

        let (instruction, operand) = parse_operation(&program[ip..])?;
        let value = operand.resolve_value(&registers);

        match instruction {
            Instruction::Adv => {
                let a = registers[Register::A];
                registers[Register::A] = a / (1 << value);
            }
            Instruction::Bxl => {
                let b = registers[Register::B];
                registers[Register::B] = b ^ value;
            }
            Instruction::Bst => {
                registers[Register::B] = value % 8;
            }
            Instruction::Jnz => {
                let a = registers[Register::A];

                if a != 0 {
                    ip = value;
                    continue;
                }
            }
            Instruction::Bxc => {
                let b = registers[Register::B];
                let c = registers[Register::C];
                registers[Register::B] = b ^ c;
            }
            Instruction::Out => {
                output.push(value % 8);
            }
            Instruction::Bdv => {
                let a = registers[Register::A];
                registers[Register::B] = a / (1 << value);

            }
            Instruction::Cdv => {
                let a = registers[Register::A];
                registers[Register::C] = a / (1 << value);
            }
        }

        ip += 2;
    }

    Ok((output, registers))
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> Result<String> {
    let (registers, program) = input;
    let (output, _) = run_program(program, *registers)?;

    Ok(output.into_iter().join(","))
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> Result<usize> {
    let (registers, program) = input;
    let mut prefixes = vec![0];

    for i in 0..program.len() {
        let mut next_prefixes = vec![];

        for prefix in prefixes {
            for candidate_suffix in 0..(1 << 3) {
                if i == 0 && candidate_suffix == 0 { continue; }

                let candidate = (prefix << 3) + candidate_suffix;
                let registers = [candidate, registers[Register::B], registers[Register::C]];
                let (output, _) = run_program(program, registers)?;

                if output[..] == program[(program.len() - (i + 1))..] {
                    next_prefixes.push(candidate);
                }
            }
        }

        prefixes = next_prefixes;
    }

    if prefixes.is_empty() {
        bail!("No solution found");
    }

    Ok(prefixes[0])
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    const EXAMPLE2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    fn run_program_fragment_1() {
        let (_, registers) = run_program(&[2, 6], [0, 0, 9]).unwrap();
        assert_eq!(&[0, 1, 9], &registers);
    }

    #[test]
    fn run_program_fragment_2() {
        let (output, _) = run_program(&[5, 0, 5, 1, 5, 4], [10, 0, 0]).unwrap();
        assert_eq!(&[0, 1, 2], &output[..]);
    }

    #[test]
    fn run_program_fragment_3() {
        let (output, registers) = run_program(&[0, 1, 5, 4, 3, 0], [2024, 0, 0]).unwrap();
        assert_eq!(&[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], &output[..]);
        assert_eq!(&[0, 0, 0], &registers);
    }

    #[test]
    fn run_program_fragment_4() {
        let (_, registers) = run_program(&[1, 7], [0, 29, 0]).unwrap();
        assert_eq!(&[0, 26, 0], &registers);
    }

    #[test]
    fn run_program_fragment_5() {
        let (_, registers) = run_program(&[4, 0], [0, 2024, 43690]).unwrap();
        assert_eq!(&[0, 44354, 43690], &registers);
    }

    #[test]
    fn part1_example1() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(&parse(EXAMPLE1).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!("1,5,3,0,2,5,2,5,3", part1(&parse(include_str!("../input/2024/day17.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_example2() {
        assert_eq!(117440, part2(&parse(EXAMPLE2).unwrap()).unwrap());
    }

    #[test]
    #[ignore]
    fn part2_input() {
        assert_eq!(108107566389757, part2(&parse(include_str!("../input/2024/day17.txt")).unwrap()).unwrap());
    }
}
