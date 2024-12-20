
use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::grid::{Direction, Grid, Position, IntoEnumIterator};
use crate::utils::path_finding::shortest_paths_to_target;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
enum MemoryCell {
    #[default]
    Uncorrupted,
    Corrupted(usize),
}

type Input = (Grid<MemoryCell>, Vec<Position>);

fn parse_with_dimensions(input: &str, height: usize, width: usize) -> Result<Input> {
    let positions = input
        .lines()
        .map(|line| {
            let (j, i) = line.split_once(',').context(format!("Unable to parse line: {line}"))?;
            Ok(Position(i.parse()?, j.parse()?))
        })
        .collect::<Result<Vec<_>>>()?;

    let corruptions = positions
        .iter()
        .enumerate()
        .map(|(time, position)| Ok((position, MemoryCell::Corrupted(time))))
        .collect::<Result<Vec<_>>>()?;

    let mut grid = Grid::new(height, width);

    for (position, memory_location) in corruptions {
        grid.set(position, memory_location);
    }

    Ok((grid, positions))
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Input> {
    parse_with_dimensions(input, 71, 71)
}

fn neighbors(grid: &Grid<MemoryCell>, position: Position, time: usize) -> Vec<(Position, usize)> {
    Direction::iter()
        .map(|direction| position.step(direction))
        .filter(|position| match grid.get(position) {
            Some(MemoryCell::Uncorrupted) => true,
            Some(MemoryCell::Corrupted(t)) => *t >= time,
            None => false,
        })
        .map(|position| (position, 1))
        .collect_vec()
}

fn distance_at_time(grid: &Grid<MemoryCell>, time: usize) -> Option<usize> {
    let source = Position(0, 0);
    let target = Position(grid.rows::<isize>() - 1, grid.cols::<isize>() - 1);
    let (distance, _) = shortest_paths_to_target(
        source,
        |position| neighbors(grid, position, time),
        |position| position == target,
    )?;

    Some(distance)
}

fn part1_with_time((grid, _): &Input, time: usize) -> Option<usize> {
    distance_at_time(grid, time)
}

#[aoc(day18, part1)]
fn part1(input: &Input) -> Option<usize> {
    part1_with_time(input, 1024)
}

#[aoc(day18, part2)]
fn part2((grid, corrupted_positions): &Input) -> Option<String> {
    let mut a = 1;
    let mut b = corrupted_positions.len() - 1;

    let time = loop {
        if a > b { break None }

        let time = (a + b) / 2;

        match (distance_at_time(grid, time - 1), distance_at_time(grid, time)) {
            (Some(_), Some(_)) => {
                a = time + 1;
            },
            (None, None) => {
                b = time - 1;
            },
            (Some(_), None) => { break Some(time) },
            (None, Some(_)) => unreachable!(),
        }
    };

    let Position(i, j) = corrupted_positions[time? - 1];
    Some(format!("{j},{i}"))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(22, part1_with_time(&parse_with_dimensions(EXAMPLE1, 7, 7).unwrap(), 12).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(408, part1(&parse(include_str!("../input/2024/day18.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_example1() {
        assert_eq!("6,1", part2(&parse_with_dimensions(EXAMPLE1, 7, 7).unwrap()).unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!("45,16", part2(&parse(include_str!("../input/2024/day18.txt")).unwrap()).unwrap());
    }
}
