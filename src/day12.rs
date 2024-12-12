use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};

use crate::utils::grid::{Direction, Position, IntoEnumIterator};

type Input = HashMap<Position, char>;

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| (Position::new(i, j), c))
        })
        .collect()
}

fn fill(map: &Input, start_position: Position) -> HashSet<Position> {
    let Some(region_plant) = map.get(&start_position) else { return HashSet::new() };
    let mut region = HashSet::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([start_position]);

    while let Some(current_position) = queue.pop_front() {
        region.insert(current_position);

        for direction in Direction::iter() {
            let next_position = current_position.step(direction);

            if !seen.insert(next_position) {
                continue;
            }

            if !map.get(&next_position).map(|plant| plant == region_plant).unwrap_or(false) {
                continue;
            }

            queue.push_back(next_position);
        }
    }

    region
}

#[aoc(day12, part1)]
fn part1(map: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from_iter(map.keys());
    let mut sum = 0;

    while let Some(&p) = queue.pop_front() {
        if seen.contains(&p) {
            continue;
        }

        let region = fill(map, p);
        for pos in &region {
            seen.insert(*pos);
        }
        let (area, perimiter) = region
            .iter()
            .fold((0, 0), |(area, perimiter), pos| {
                let p = Direction::iter()
                    .map(|d| pos.step(d))
                    .filter(|np| !region.contains(np))
                    .count();
                (area + 1, perimiter + p)
            });

        sum += area * perimiter;
    }

    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const EXAMPLE2: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(140, part1(&parse(EXAMPLE1)));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(1930, part1(&parse(EXAMPLE2)));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1433460, part1(&parse(include_str!("../input/2024/day12.txt"))));
    }
}
