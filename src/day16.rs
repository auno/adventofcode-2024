use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, VecDeque}, iter::repeat};

use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

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

fn resolve_paths(
    distances: &HashMap<(Position, Direction), (usize, Vec<(Position, Direction)>)>,
    targets: &[(Position, Direction)],
    source: (Position, Direction),
) -> Vec<VecDeque<(Position, Direction)>> {
    let target_previous_positions = targets
        .iter()
        .map(|target| {
            if *target == source {
                (*target, vec![])
            } else {
                (
                    *target,
                    distances
                        .get(target)
                        .map(|(_, prev)| prev)
                        .unwrap_or_else(|| panic!("Unable to find previous step: {target:?}"))
                        .clone()
                )
            }
        })
        .collect_vec();

    target_previous_positions
        .into_iter()
        .map(|(target, previous_positions)| {
            (target, resolve_paths(distances, &previous_positions, source))
        })
        .flat_map(|(target, previous_paths)| {
            if previous_paths.is_empty() {
                return vec![VecDeque::from([target])];
            }

            previous_paths
                .into_iter()
                .update(move |path| {
                    path.push_front(target);
                })
                .collect_vec()
        })
        .collect_vec()
}

fn distance(grid: &Grid<Tile>, source: (Position, Direction), target: Position) -> Option<(usize, Vec<VecDeque<(Position, Direction)>>)> {
    let mut distances: HashMap<(Position, Direction), (usize, Vec<(Position, Direction)>)> = HashMap::from([(source, (0, vec![]))]);
    let mut queue: BinaryHeap<(Reverse<usize>, (Position, Direction))> = BinaryHeap::from([(Reverse(0), source)]);

    while let Some((Reverse(distance), (position, direction))) = queue.pop() {
        if position == target {
            break;
        }

        for (neighbor, cost) in neighbors(grid, position, direction) {
            let (neighbor_distance, neighbor_source) = distances
                .entry(neighbor)
                .or_insert((usize::MAX, vec![]));

            match (distance + cost).cmp(neighbor_distance) {
                Ordering::Less => {
                    *neighbor_distance = distance + cost;
                    *neighbor_source = vec![(position, direction)];
                    queue.push((Reverse(*neighbor_distance), neighbor));
                }
                Ordering::Equal => {
                    neighbor_source.push((position, direction));
                }
                Ordering::Greater => {},
            }
        }
    }

    let min_distance = Direction::iter()
        .filter_map(|direction| distances.get(&(target, direction)))
        .map(|(distance, _)| *distance)
        .min()?;

    let targets = repeat(target)
        .zip(Direction::iter())
        .filter(|target| distances.get(target).map(|(distance, _)| *distance) == Some(min_distance))
        .collect_vec();

    Some((min_distance, resolve_paths(&distances, &targets, source)))
}

#[aoc(day16, part1)]
fn part1((grid, start, goal): &Input) -> Option<usize> {
    let (distance, _) = distance(grid, (*start, Direction::Right), *goal)?;
    Some(distance)
}

#[aoc(day16, part2)]
fn part2((grid, start, goal): &Input) -> Option<usize> {
    let (_, paths) = distance(grid, (*start, Direction::Right), *goal)?;
    let num_positions = paths
            .into_iter()
            .flat_map(|paths| paths.into_iter())
            .map(|(position, _)| {
                position
            })
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
