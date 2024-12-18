
use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::grid::{Direction, Grid, Position};
use crate::utils::path_finding::{distance, PathMap};

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

type SearchNode = (Position, Direction);

fn neighbors(grid: &Grid<Tile>, (position, direction): SearchNode) -> Vec<(SearchNode, usize)> {
    [(direction, 1), (direction.turn(), 1001), (direction.turn().turn(), 2001), (direction.turn().turn().turn(), 1001)]
        .into_iter()
        .map(|(direction, cost)| ((position.step(direction), direction), cost))
        .filter(|((position, _), _)| grid.get(position) == Some(&Tile::Free))
        .collect::<Vec<_>>()
}

fn shortest_paths((grid, start, goal): &Input) -> Option<(usize, PathMap<SearchNode>)> {
    distance(
        (*start, Direction::Right),
        |node| neighbors(grid, node),
        |(position, _) | position == *goal,
    )
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> Option<usize> {
    let (distance, _) = shortest_paths(input)?;
    Some(distance)
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> Option<usize> {
    let (_, path_map) = shortest_paths(input)?;

    let num_positions = path_map
        .into_iter()
        .map(|((position, _), _)| position)
        .unique()
        .count();

    Some(num_positions)
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

    #[test]
    fn part2_example1() {
        assert_eq!(Some(45), part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(Some(64), part2(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(Some(483), part2(&parse(include_str!("../input/2024/day16.txt")).unwrap()));
    }
}
