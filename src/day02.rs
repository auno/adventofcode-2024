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

fn is_valid_report<'a>(report: impl Iterator<Item = &'a u32>) -> bool {
    use std::cmp::Ordering::Equal;
    use itertools::FoldWhile::{Continue, Done};

    report
        .tuple_windows()
        .map(|(a, b)| (a.cmp(b), a.abs_diff(*b)))
        .fold_while(None, |existing_ordering, (ordering, diff)| {
            if diff > 3 {
                return Done(None);
            }

            match (ordering, existing_ordering) {
                (Equal, _) => Done(None),
                (_, None) => Continue(Some(ordering)),
                (a, Some(b)) if a == b => Continue(Some(ordering)),
                (_, Some(_)) => Done(None),
            }
        })
        .into_inner()
        .is_some()
}

fn solve(reports: &[Vec<u32>], test: impl Fn(&[u32]) -> bool) -> usize {
    reports
        .iter()
        .filter(|report| test(report))
        .count()
}

#[aoc(day2, part1)]
fn part1(reports: &[Vec<u32>]) -> usize {
    solve(reports, |report| is_valid_report(report.iter()))
}

#[aoc(day2, part2)]
fn part2(reports: &[Vec<u32>]) -> usize {
    solve(reports, |report| {
        (0..report.len()).any(|i| {
            is_valid_report(
                report
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, l)| l)
            )
        })
    })
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

    #[test]
    fn part2_example1() {
        assert_eq!(4, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(455, part2(&parse(include_str!("../input/2024/day2.txt")).unwrap()));
    }
}
