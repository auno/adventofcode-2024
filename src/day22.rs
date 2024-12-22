use std::iter;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<i64>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| line.parse().context(format!("Unable to parse line: {line}")))
        .collect()
}

fn secrets(initial_secret: i64) -> impl Iterator<Item = i64> {
    iter::successors(Some(initial_secret), |previous| {
        let mut secret = *previous;

        secret = ((secret << 6) ^ secret) % 16777216;
        secret = ((secret >> 5) ^ secret) % 16777216;
        secret = ((secret << 11) ^ secret) % 16777216;

        Some(secret)
    })
}

fn prices(initial_secret: i64) -> impl Iterator<Item = i64> {
    secrets(initial_secret)
        .map(|secret| secret % 10)
}

fn pattern_index(a: i64, b: i64, c: i64, d: i64, e: i64) -> usize {
    [b - a, c - b, d - c, e - d]
        .into_iter()
        .map(|change| (change + 9) as usize)
        .fold(0, |acc, v| {
            assert!(v < (19));
            (acc * 19) + v
        })
}

#[aoc(day22, part1)]
fn part1(initial_secrets: &Input) -> i64 {
    initial_secrets
        .iter()
        .filter_map(|initial| secrets(*initial).nth(2000))
        .sum()
}

#[aoc(day22, part2)]
fn part2(initial_secrets: &Input) -> usize {
    let mut bananas = vec![0; 130321];

    for &initial_secret in initial_secrets {
        let mut seen = vec![false; 130321];

        for (a, b, c, d, e) in prices(initial_secret).take(2001).tuple_windows() {
            let pattern = pattern_index(a, b, c, d, e);
            if seen[pattern] { continue; }
            bananas[pattern] += e as usize;
            seen[pattern] = true;
        }
    }

    bananas.into_iter().max().unwrap()
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

    const EXAMPLE2: &str = indoc! {"
        1
        2
        3
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

    #[test]
    fn part2_example2() {
        assert_eq!(23, part2(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1896, part2(&parse(include_str!("../input/2024/day22.txt")).unwrap()));
    }
}
