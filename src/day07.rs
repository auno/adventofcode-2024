use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => {
                let mut shifted = a;
                let mut unshifted = b;

                while unshifted > 0 {
                    shifted *= 10;
                    unshifted /= 10;
                }

                shifted + b
            }
        }
    }
}

type Input = Vec<(u64, Vec<u64>)>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split([' ', ':'])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().context(format!("Unable to parse number: {s}")));

            let test_value = numbers
                .next()
                .context("No test value found")??;

            let operands = numbers.collect::<Result<_>>()?;

            Ok((test_value, operands))
        })
        .collect()
}

fn has_valid_solution(test_value: u64, result: u64, operands: &[u64], operators: &[Operator]) -> bool {
    if operands.is_empty() { return result == test_value; }
    if result > test_value { return false; }

    let operand = operands[0];

    for operator in operators {
        if has_valid_solution(test_value, operator.apply(result, operand), &operands[1..], operators) {
            return true;
        }
    }

    false
}

fn solve(input: &Input, operators: &[Operator]) -> u64 {
    input
        .iter()
        .filter_map(|(test_value, operands)| {
            if has_valid_solution(*test_value, *operands.first()?, &operands[1..], operators) {
                return Some(test_value);
            }

            None
        })
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> u64 {
    solve(input, &[Operator::Add, Operator::Multiply])
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> u64 {
    solve(input, &[Operator::Add, Operator::Multiply, Operator::Concatenate])
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(3749, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1298300076754, part1(&parse(include_str!("../input/2024/day7.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(11387, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(248427118972289, part2(&parse(include_str!("../input/2024/day7.txt")).unwrap()));
    }
}
