use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::utils::grid::{Direction, Grid, Position};

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
type Distances = HashMap<SearchNode, (usize, Vec<SearchNode>)>;
type PathMap = HashMap<SearchNode, Vec<SearchNode>>;

fn neighbors(grid: &Grid<Tile>, (position, direction): SearchNode) -> impl IntoIterator<Item = (SearchNode, usize)> {
    [(direction, 1), (direction.turn(), 1001), (direction.turn().turn(), 2001), (direction.turn().turn().turn(), 1001)]
        .into_iter()
        .map(|(direction, cost)| ((position.step(direction), direction), cost))
        .filter(|((position, _), _)| grid.get(position) == Some(&Tile::Free))
        .collect::<Vec<_>>()
}

fn resolve_path_map(distances: &Distances, targets: &[SearchNode]) -> PathMap {
    let mut queue = VecDeque::from_iter(targets.iter().copied());
    let mut seen = HashSet::new();
    let mut path_map = HashMap::from_iter(targets.iter().map(|target| (*target, vec![])));

    while let Some(current) = queue.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        for &previous in distances
            .get(&current)
            .map(|(_, previous)| previous)
            .unwrap_or(&vec![])
        {
            path_map.entry(previous).or_default().push(current);
            queue.push_back(previous);
        }
    }

    path_map
}

fn distance(grid: &Grid<Tile>, source: SearchNode, is_target: impl Fn(SearchNode) -> bool) -> Option<(usize, PathMap)> {
    let mut distances = HashMap::from([(source, (0, vec![]))]);
    let mut queue = BinaryHeap::from([(Reverse(0), source)]);

    while let Some((Reverse(distance), current)) = queue.pop() {
        if is_target(current) {
            break;
        }

        for (neighbor, cost) in neighbors(grid, current) {
            let (neighbor_distance, neighbor_source) = distances
                .entry(neighbor)
                .or_insert((usize::MAX, vec![]));

            match (distance + cost).cmp(neighbor_distance) {
                Ordering::Less => {
                    *neighbor_distance = distance + cost;
                    *neighbor_source = vec![current];
                    queue.push((Reverse(*neighbor_distance), neighbor));
                }
                Ordering::Equal => {
                    neighbor_source.push(current);
                }
                Ordering::Greater => {},
            }
        }
    }

    let potential_targets = distances
        .iter()
        .filter(|(node, _)| is_target(**node))
        .collect_vec();

    let min_distance = potential_targets
        .iter()
        .map(|(_, (distance, _))| *distance)
        .min()?;

    let targets = potential_targets
        .iter()
        .filter(|(_, (distance, _))| *distance == min_distance)
        .map(|(node, _)| **node)
        .collect_vec();

    Some((min_distance, resolve_path_map(&distances, &targets)))
}

#[aoc(day16, part1)]
fn part1((grid, start, goal): &Input) -> Option<usize> {
    let (distance, _) = distance(grid, (*start, Direction::Right), |(position, _): SearchNode| position == *goal)?;
    Some(distance)
}

#[aoc(day16, part2)]
fn part2((grid, start, goal): &Input) -> Option<usize> {
    let (_, path_map) = distance(grid, (*start, Direction::Right), |(position, _): SearchNode| position == *goal)?;

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
