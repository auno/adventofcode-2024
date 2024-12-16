use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

use crate::utils::grid::{Direction, Grid, Position, IntoEnumIterator};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Free,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Free),
            '#' => Ok(Tile::Wall),
            _ => bail!("Unable to parse Tile: {value}"),
        }
    }
}

type Input = (Grid<Tile>, Position, Position);

#[aoc_generator(day16)]
fn parse(input: &str) -> Result<Input> {
    let (grid, positions) = Grid::<Tile>::parse_with_position_detection(input, &['S', 'E'], Tile::Free)?;
    let Some(&[start]) = positions.get(&'S').map(Vec::as_slice) else {
        bail!("Could not parse start position: {positions:?}");
    };
    let Some(&[goal]) = positions.get(&'E').map(Vec::as_slice) else {
        bail!("Could not parse goal position: {positions:?}");
    };

    Ok((grid, start, goal))
}

fn neighbors(grid: &Grid<Tile>, position: Position, direction: Direction) -> impl IntoIterator<Item = ((Position, Direction), usize)> {
    [(direction, 1), (direction.turn(), 1001), (direction.turn().turn(), 2001), (direction.turn().turn().turn(), 1001)]
        .into_iter()
        .map(|(direction, cost)| ((position.step(direction), direction), cost))
        .filter(|((position, _), _)| grid.get(position) == Some(&Tile::Free))
        .collect::<Vec<_>>()
}

fn distance(grid: &Grid<Tile>, source: (Position, Direction), target: Position) -> Option<usize> {
    let mut distances: HashMap<(Position, Direction), usize> = HashMap::from([(source, 0)]);
    let mut queue: BinaryHeap<(Reverse<usize>, (Position, Direction))> = BinaryHeap::from([(Reverse(0), source)]);

    while let Some((Reverse(distance), (position, direction))) = queue.pop() {
        if position == target {
            break;
        }

        for (neighbor, cost) in neighbors(grid, position, direction) {
            let neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

            if *neighbor_distance > distance + cost {
                *neighbor_distance = distance + cost;
                queue.push((Reverse(*neighbor_distance), neighbor));
            }
        }
    }

    Direction::iter().filter_map(|direction| distances.get(&(target, direction))).copied().min()
}

#[aoc(day16, part1)]
fn part1((grid, start, goal): &Input) -> Option<usize> {
    distance(grid, (*start, Direction::Right), *goal)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    const EXAMPLE2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(Some(7036), part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(Some(11048), part1(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(83444), part1(&parse(include_str!("../input/2024/day16.txt")).unwrap()));
    }
}
