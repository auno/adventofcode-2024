use std::{collections::HashMap, usize};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Position(isize, isize);

impl Position {
    fn new(i: usize, j: usize) -> Position {
        Position(i as isize, j as isize)
    }

    fn step(self, dir: Direction) -> Position {
        let Position(i, j) = self;
        match dir {
            Direction::Up => Position(i - 1, j),
            Direction::Down => Position(i + 1, j),
            Direction::Left => Position(i, j - 1),
            Direction::Right => Position(i, j + 1),
        }
    }
}

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

fn count_reachable_summits(map: &Input, p: &Position) -> impl IntoIterator<Item = Position> {
    let current_height = map.get(p).unwrap();

    if *current_height == 9 {
        // eprintln!("-- {p:?}");
        return vec![*p];
    }

    Direction::iter()
        .filter_map(|d| {
            let np = p.step(d);
            let nh = map.get(&np)?;

            if *nh != current_height + 1 {
                return None;
            }

            Some(count_reachable_summits(map, &np))
        })
        .flatten()
        .collect_vec()
}

#[aoc(day10, part1)]
fn part1(map: &Input) -> usize {
    let trailheads = map
        .iter()
        .filter(|(p, h)| **h == 0)
        .map(|(p, _)| *p)
        .collect_vec();

    trailheads
        .iter()
        .map(|p| count_reachable_summits(map, p).into_iter().unique().count())
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
}
