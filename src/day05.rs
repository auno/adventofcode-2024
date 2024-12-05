use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Rules = Vec<(u32, u32)>;
type Updates = Vec<Vec<u32>>;

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<(Rules, Updates)> {
    let (rules, updates) = input.split_once("\n\n").context("Unable to parse input")?;

    let rules = rules
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("|").context(format!("Unable to parse rule: {line}"))?;
            let l = l.parse::<u32>()?;
            let r = r.parse::<u32>()?;

            Ok((l, r))
        })
        .collect::<Result<_>>()?;

    let updates = updates
        .lines()
        .map(|line| {
            line
                .split(",")
                .map(|page| page.parse::<u32>().context(format!("Unable to parse page number: {page}")))
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()?;

    Ok((rules, updates))
}

#[aoc(day5, part1)]
fn part1((rules, updates): &(Rules, Updates)) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            let indices = update
                .iter()
                .enumerate()
                .map(|(i, p)| (*p, i))
                .collect::<HashMap<_,_>>();

            let order_correct = rules
                .iter()
                .filter_map(|(l, r)| {
                    let li = indices.get(l)?;
                    let ri = indices.get(r)?;

                    Some((li, ri))
                })
                .all(|(li, ri)| li < ri);

            if !order_correct {
                return None;
            }

            update.get(update.len() / 2)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(143, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6949, part1(&parse(include_str!("../input/2024/day5.txt")).unwrap()));
    }
}
