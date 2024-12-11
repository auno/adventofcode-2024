use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u64>().context(format!("Unable to parse number: {n}")))
        .collect()
}

fn num_digits(stone: u64) -> u32 {
    let mut stone = stone;
    let mut digits = 0;

    while stone > 0 {
        stone /= 10;
        digits += 1;
    }

    digits
}

fn map_stone(stone: u64) -> [Option<u64>; 2] {
    if stone == 0 {
        return [Some(1), None];
    }

    let digits = num_digits(stone);
    if digits % 2 == 0 {
        let pow = 10u64.pow(digits / 2);
        let a = stone / pow;
        let b = stone - (a * pow);

        return [Some(a), Some(b)];
    }

    [Some(stone * 2024), None]
}

#[cached]
fn count_stones(stone: u64, iterations: usize) -> usize {
    if iterations == 0 {
        return 1;
    }

    map_stone(stone)
        .into_iter()
        .flatten()
        .map(|new_stone| count_stones(new_stone, iterations - 1))
        .sum()
}

fn solve(stones: &[u64], iterations: usize) -> usize {
    stones
        .iter()
        .map(|stone| count_stones(*stone, iterations))
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
    fn num_digits_1() {
        assert_eq!(6, num_digits(253000));
    }

    #[test]
    fn map_stone_1() {
        assert_eq!([Some(253), Some(0)], map_stone(253000));
    }

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
