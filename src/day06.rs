use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

enum Tile {
    Free,
    Obstructed,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Tile::Free),
            '#' => Ok(Tile::Obstructed),
            _ => bail!("Unknown Tile: {value}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Position(isize, isize);

impl Position {
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

type Map = HashMap<Position, Tile>;

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<(Map, Position)> {
    let (map, guard_pos) = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line
            .chars()
            .enumerate()
            .map(move |(j, c)| (Position(i as isize, j as isize), c))
        )
        .map(|(pos, c)| -> Result<(Position, Tile, bool)> {
            match c {
                '^' => Ok((pos, Tile::Free, true)),
                _ => Ok((pos, Tile::try_from(c)?, false)),
            }
        })
        .fold_ok((HashMap::<_,_>::new(), None), |(mut acc, guard_pos), (pos, tile, is_guard_pos )| {
            acc.insert(pos, tile);
            (acc, if is_guard_pos { Some(pos) } else { guard_pos })
        })?;

    Ok((map, guard_pos.context("No guard position found")?))
}

#[aoc(day6, part1)]
fn part1((map, guard_pos): &(Map, Position)) -> usize {
    let mut seen = HashSet::from([*guard_pos]);
    let mut position = *guard_pos;
    let mut direction = Direction::Up;

    loop {
        let candidate_position = position.step(direction);
        match map.get(&candidate_position) {
            Some(Tile::Free) => {
                position = candidate_position;
                seen.insert(candidate_position);
            },
            Some(Tile::Obstructed) => {
                direction = direction.turn();
            },
            None => break,
        }
    }

    seen.len()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(41, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(5095, part1(&parse(include_str!("../input/2024/day6.txt")).unwrap()));
    }
}
