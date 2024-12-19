use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::path_finding::distance;

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
fn parse(input: &str) -> Result<Input> {
    let (available, desired) = input.split_once("\n\n").context("Could not parse input")?;
    let available = available.split(", ").map(str::to_string).collect();
    let desired = desired.lines().map(str::to_string).collect();

    Ok((available, desired))
}

fn neighbors(available_patterns: &[String], desired_pattern: &str, i: usize) -> Vec<(usize, usize)> {
    available_patterns
        .iter()
        .filter_map(|candidate| {
            match desired_pattern[i..].starts_with(candidate) {
                true => Some((i + candidate.len(), 1)),
                false => None,
            }
        })
        .collect()
}

#[aoc(day19, part1)]
fn part1((available_patterns, desired_pattern): &Input) -> usize {
    desired_pattern
        .iter()
        .filter(|desired_pattern| {
            distance(
                0,
                |i| neighbors(available_patterns, desired_pattern, i),
                |i| i == desired_pattern.len()
            ).is_some()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(6, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(304, part1(&parse(include_str!("../input/2024/day19.txt")).unwrap()));
    }
}
