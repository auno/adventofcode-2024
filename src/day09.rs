use std::iter::repeat_n;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<Option<usize>>> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).map(|d| (i, d as usize)).context(format!("Unable to parse block size: {c}")))
        .map_ok(|(i, len)| {
            let element = if i % 2 == 0 { Some(i / 2) } else { None };
            repeat_n(element, len)
        })
        .flatten_ok()
        .collect()
}

#[aoc(day9, part1)]
fn part1(disk_map: &[Option<usize>]) -> usize {
    let mut compacted = disk_map.to_vec();

    let mut a = 0;
    let mut b = compacted.len() - 1;

    while b > a {
        match (compacted[a], compacted[b]) {
            (Some(_), _) => {
                a += 1;
            },
            (None, Some(_)) => {
                compacted.swap(a, b);
                a += 1;
                b -= 1;
            },
            (None, None) => {
                b -= 1;
            },
        }
    }

    compacted
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|f| i * f))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "2333133121414131402";

    #[test]
    fn parse_example1() {
        let parsed = parse(EXAMPLE1).unwrap();
        let string_rep = parsed
            .iter()
            .map(|b| match b {
                Some(f) => f.to_string(),
                None => ".".to_string(),
            })
            .join("");
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", string_rep);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(1928, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6378826667552, part1(&parse(include_str!("../input/2024/day9.txt")).unwrap()));
    }
}
