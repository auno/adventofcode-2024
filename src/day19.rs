use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
fn parse(input: &str) -> Result<Input> {
    let (available, desired) = input.split_once("\n\n").context("Could not parse input")?;
    let available = available.split(", ").map(str::to_string).collect();
    let desired = desired.lines().map(str::to_string).collect();

    Ok((available, desired))
}

fn count_combinations(cache: &mut [Option<usize>], available_patterns: &[String], desired_pattern: &str) -> usize {
    if desired_pattern.is_empty() {
        return 1;
    }

    if let Some(result) = cache[0] {
        return result;
    }

    let result = available_patterns
        .iter()
        .map(|candidate| {
            match desired_pattern.starts_with(candidate) {
                true => count_combinations(&mut cache[candidate.len()..], available_patterns, &desired_pattern[candidate.len()..]),
                false => 0,
            }
        })
        .sum();

    cache[0] = Some(result);
    result
}

#[aoc(day19, part1)]
fn part1((available_patterns, desired_patterns): &Input) -> usize {
    desired_patterns
        .iter()
        .filter(|desired_pattern| {
            let mut cache = vec![None; desired_pattern.len()];
            count_combinations(&mut cache, available_patterns, desired_pattern) > 0
        })
        .count()
}

#[aoc(day19, part2)]
fn part2((available_patterns, desired_patterns): &Input) -> usize {
    desired_patterns
        .iter()
        .map(|desired_pattern| {
            let mut cache = vec![None; desired_pattern.len()];
            count_combinations(&mut cache, available_patterns, desired_pattern)
        })
        .sum()
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

    #[test]
    fn part2_example1() {
        assert_eq!(16, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(705756472327497, part2(&parse(include_str!("../input/2024/day19.txt")).unwrap()));
    }
}
