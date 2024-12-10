use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::grid::{Direction, Position, IntoEnumIterator};

type Input = HashMap<Position, usize>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)|
            line.chars()
                .enumerate()
                .map(move |(j, c)| Ok((
                    Position::new(i, j),
                    c.to_digit(10).context(format!("Unable to parse height: {c}"))? as usize,
                )))
        )
        .collect()
}

fn paths_to_summit(map: &Input, p: Position) -> impl IntoIterator<Item = Position> {
    let current_height = map.get(&p).unwrap();

    if *current_height == 9 {
        return vec![p];
    }

    Direction::iter()
        .filter_map(|d| {
            let np = p.step(d);
            let nh = map.get(&np)?;

            if *nh != current_height + 1 {
                return None;
            }

            Some(paths_to_summit(map, np))
        })
        .flatten()
        .collect_vec()
}

fn trailheads(map: &Input) -> impl IntoIterator<Item = &'_ Position> {
    map
        .iter()
        .filter(|(_, h)| **h == 0)
        .map(|(p, _)| p)
}

#[aoc(day10, part1)]
fn part1(map: &Input) -> usize {
    trailheads(map)
        .into_iter()
        .map(|p| paths_to_summit(map, *p).into_iter().unique().count())
        .sum()
}

#[aoc(day10, part2)]
fn part2(map: &Input) -> usize {
    trailheads(map)
        .into_iter()
        .map(|p| paths_to_summit(map, *p).into_iter().count())
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        0123
        1234
        8765
        9876
    "};

    const EXAMPLE2: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(1, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(36, part1(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(820, part1(&parse(include_str!("../input/2024/day10.txt")).unwrap()));
    }


    #[test]
    fn part2_example2() {
        assert_eq!(81, part2(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1786, part2(&parse(include_str!("../input/2024/day10.txt")).unwrap()));
    }
}
