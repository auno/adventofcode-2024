use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::grid::{Direction, Grid, Position, IntoEnumIterator};
use crate::utils::path_finding::{distance_to_target, shortest_paths_to_target};

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

#[aoc_generator(day20)]
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

type SearchNode = Position;

fn neighbors(grid: &Grid<Tile>, position: SearchNode) -> Vec<(SearchNode, usize)> {
    Direction::iter()
        .map(|direction| (position.step(direction), 1))
        .filter(|(position, _)| grid.get(position) == Some(&Tile::Free))
        .collect()
}

fn part1_with_limit((grid, start, goal): &Input, limit: usize) -> Option<usize> {
    let (non_cheat_distance, path_map) = shortest_paths_to_target(
        *start,
        |position| neighbors(grid, position),
        |position| position == *goal,
    )?;

    let cheats = path_map
        .keys()
        .flat_map(|position| {
            Direction::iter()
                .map(|direction| (*position, position.step(direction), position.step(direction).step(direction)))
                .filter(|(_, p1, p2)| grid.get(p1) == Some(&Tile::Wall) && grid.get(p2) == Some(&Tile::Free))
        })
        .collect_vec();

    let num_valid_cheats = cheats
        .into_iter()
        .filter_map(|(cheat_source, _, cheat_target)| {
            let cheat_distance = distance_to_target(
                *start,
                |position| {
                    let mut neighbors = neighbors(grid, position);

                    if position == cheat_source {
                        neighbors.push((cheat_target, 2));
                    }

                    neighbors
                },
                |position| position == *goal,
            )?;

            Some(cheat_distance)
        })
        .filter(|distance| non_cheat_distance - distance >= limit)
        .count();

    Some(num_valid_cheats)
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> Option<usize> {
    part1_with_limit(input, 100)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn part1_example1_1() {
        assert_eq!(Some(44), part1_with_limit(&parse(EXAMPLE1).unwrap(), 2));
    }

    #[test]
    fn part1_example1_2() {
        assert_eq!(Some(14), part1_with_limit(&parse(EXAMPLE1).unwrap(), 8));
    }

    #[test]
    fn part1_example1_3() {
        assert_eq!(Some(1), part1_with_limit(&parse(EXAMPLE1).unwrap(), 64));
    }

    #[test]
    #[ignore]
    fn part1_input() {
        assert_eq!(17, part1(&parse(include_str!("../input/2024/day20.txt")).unwrap()).unwrap());
    }
}
