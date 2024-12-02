use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|num| num.parse().context(format!("Unable to parse line: {line}")))
                .collect::<Result<Vec<u32>>>()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .filter(|report| report.iter().is_sorted() || report.iter().rev().is_sorted())
        .filter(|report| report.iter().tuple_windows().all(|(&a, &b)| (1..=3).contains(&a.abs_diff(b))))
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(2, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(402, part1(&parse(include_str!("../input/2024/day2.txt")).unwrap()));
    }
}
