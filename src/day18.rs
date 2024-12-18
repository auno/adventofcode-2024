use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::utils::grid::{Direction, Grid, Position, IntoEnumIterator};

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

type SearchNode = (Position, usize);
type Distances = HashMap<SearchNode, (usize, Vec<SearchNode>)>;
type PathMap = HashMap<SearchNode, Vec<SearchNode>>;

fn neighbors(grid: &Grid<MemoryCell>, (position, time): SearchNode) -> Vec<(SearchNode, usize)> {
    Direction::iter()
        .map(|direction| position.step(direction))
        .filter(|position| match grid.get(position) {
            Some(MemoryCell::Uncorrupted) => true,
            Some(MemoryCell::Corrupted(t)) => *t >= time,
            None => false,
        })
        .map(|position| ((position, time + 1), 1))
        .collect_vec()
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

fn distance(
    grid: &Grid<MemoryCell>,
    source: SearchNode,
    neighbors: impl Fn(&Grid<MemoryCell>, SearchNode) -> Vec<(SearchNode, usize)>,
    is_target: impl Fn(SearchNode) -> bool
) -> Option<(usize, PathMap)> {
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

fn distance_at_time(grid: &Grid<MemoryCell>, time: usize) -> Option<usize> {
    let source = (Position(0, 0), 0);
    let target = Position(grid.rows::<isize>() - 1, grid.cols::<isize>() - 1);
    let (distance, _) = distance(
        grid,
        source,
        |grid, (position, _)| neighbors(grid, (position, time)),
        |(p, _)| p == target,
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
    for time in 1..corrupted_positions.len() {
        if distance_at_time(grid, time).is_none() {
            let Position(i, j) = corrupted_positions[time - 1];
            return Some(format!("{j},{i}"));
        }
    }

    None
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
    #[ignore]
    fn part2_input() {
        assert_eq!("45,16", part2(&parse(include_str!("../input/2024/day18.txt")).unwrap()).unwrap());
    }
}
