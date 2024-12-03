use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> Result<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    pattern
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a = a.parse::<u32>()?;
            let b = b.parse::<u32>()?;

            Ok(a * b)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example1() {
        assert_eq!(161, part1(&parse(EXAMPLE1)).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(184576302, part1(&parse(include_str!("../input/2024/day3.txt"))).unwrap());
    }
}
