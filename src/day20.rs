use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::grid::{Direction, Grid, Position, IntoEnumIterator};
use crate::utils::path_finding::shortest_paths_to_target;

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

fn solve((grid, start, goal): &Input, maximum_cheat_length: usize, minimum_saved_time: usize) -> Option<usize> {
    let maximum_cheat_length = maximum_cheat_length as isize;
    let minimum_saved_time = minimum_saved_time as isize;

    let (non_cheat_distance, path_map) = shortest_paths_to_target(
        *start,
        |position| neighbors(grid, position),
        |position| position == *goal,
    )?;

    let mut distance_to_target = Grid::new_with_value(grid.rows::<isize>(), grid.cols(), None);
    let mut path = vec![];
    let mut current = Some((*start, non_cheat_distance as isize));

    while let Some((position, distance)) = current {
        path.push(position);
        distance_to_target.set(&position, Some(distance));

        if distance == 0 { break };

        current = path_map.get(&position).map(|next| {
            assert_eq!(1, next.len());
            (next[0], distance - 1)
        });
    }

    let mut count = 0;

    for cheat_source in path {
        let Position(i, j) = cheat_source;
        let cheat_source_distance = distance_to_target.get(&cheat_source).unwrap().unwrap();

        for oi in -maximum_cheat_length..=maximum_cheat_length {
            for oj in -(maximum_cheat_length - oi.abs())..=(maximum_cheat_length - oi.abs()) {
                let cheat_length = oi.abs() + oj.abs();
                let cheat_target = Position(i + oi, j + oj);
                let Some(Some(cheat_target_distance)) = distance_to_target.get(&cheat_target) else { continue };

                if cheat_source_distance - (cheat_target_distance + cheat_length) >= minimum_saved_time {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> Option<usize> {
    solve(input, 2, 100)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> Option<usize> {
    solve(input, 20, 100)
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
        assert_eq!(Some(44), solve(&parse(EXAMPLE1).unwrap(), 2, 2));
    }

    #[test]
    fn part1_example1_2() {
        assert_eq!(Some(14), solve(&parse(EXAMPLE1).unwrap(), 2, 8));
    }

    #[test]
    fn part1_example1_3() {
        assert_eq!(Some(1), solve(&parse(EXAMPLE1).unwrap(), 2, 64));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1518, part1(&parse(include_str!("../input/2024/day20.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_example1_1() {
        assert_eq!(Some(285), solve(&parse(EXAMPLE1).unwrap(), 20, 50));
    }

    #[test]
    fn part2_example1_2() {
        assert_eq!(Some(29), solve(&parse(EXAMPLE1).unwrap(), 20, 72));
    }

    #[test]
    fn part2_example1_3() {
        assert_eq!(Some(3), solve(&parse(EXAMPLE1).unwrap(), 20, 76));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1032257, part2(&parse(include_str!("../input/2024/day20.txt")).unwrap()).unwrap());
    }
}
