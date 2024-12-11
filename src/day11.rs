use hashbrown::HashMap;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u64>().context(format!("Unable to parse number: {n}")))
        .collect()
}

fn num_digits(number: u64) -> usize {
    let mut number = number;
    let mut digits = 0;

    while number > 0 {
        number /= 10;
        digits += 1;
    }

    digits
}

fn solve(stones: &[u64], iterations: usize) -> usize {
    let mut stone_counts = stones
        .iter()
        .fold(HashMap::<u64, usize>::new(), |mut acc, stone| {
            *acc.entry(*stone).or_default() += 1;
            acc
        });

    for _ in 0..iterations {
        let mut new_stone_counts = HashMap::with_capacity(stone_counts.capacity());

        for (stone, count) in stone_counts {
            if stone == 0 {
                *new_stone_counts.entry(1).or_default() += count;
                continue;
            }

            let digits = num_digits(stone);
            if digits % 2 == 0 {
                let pow = 10u64.pow(digits as u32 / 2);
                let a = stone / pow;
                let b = stone % pow;

                *new_stone_counts.entry(a).or_default() += count;
                *new_stone_counts.entry(b).or_default() += count;

                continue;
            }

            *new_stone_counts.entry(stone * 2024).or_default() += count;
        }

        stone_counts = new_stone_counts;
    }

    stone_counts
        .values()
        .sum()
}

#[aoc(day11, part1)]
fn part1(stones: &[u64]) -> usize {
    solve(stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones: &[u64]) -> usize {
    solve(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "125 17";

    #[test]
    fn part1_example1() {
        assert_eq!(55312, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(204022, part1(&parse(include_str!("../input/2024/day11.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(241651071960597, part2(&parse(include_str!("../input/2024/day11.txt")).unwrap()));
    }
}
