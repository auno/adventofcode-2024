use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<usize>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| line.parse().context(format!("Unable to parse line: {line}")))
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|number| {
            let mut number = *number;

            for _ in 0..2000 {
                number = ((number << 6) ^ number) % 16777216;
                number = ((number >> 5) ^ number) % 16777216;
                number = ((number << 11) ^ number) % 16777216;
            }

            number
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        1
        10
        100
        2024
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(37327623, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(16299144133, part1(&parse(include_str!("../input/2024/day22.txt")).unwrap()));
    }
}
