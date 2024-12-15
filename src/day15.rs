use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::grid::{Direction, Grid, Position};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Free,
    Box,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Free),
            'O' => Ok(Tile::Box),
            '#' => Ok(Tile::Wall),
            _ => bail!("Unable to parse Tile: {value}"),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Free => '.',
            Tile::Box => 'O',
            Tile::Wall => '#',
        }
    }
}

type Input = (Grid<Tile>, Position, Vec<Direction>);

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Input> {
    let (grid, movements) = input.split_once("\n\n").context("Unable to parse input")?;

    let (grid, start_position) = Grid::parse_with_start_position(grid, '@', Tile::Free)?;
    let movements = movements
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::try_from)
        .collect::<Result<_>>();

    Ok((grid, start_position, movements?))
}

fn find_free_position(grid: &Grid<Tile>, position: Position, direction: Direction) -> Option<Position> {
    let mut position = position;

    loop {
        position = position.step(direction);

        let Some(tile) = grid.get(&position) else { break };

        match tile {
            Tile::Free => return Some(position),
            Tile::Wall => return None,
            Tile::Box => {},
        }
    }

    None
}

#[aoc(day15, part1)]
fn part1((grid, start_position, movements): &Input) -> usize {
    let mut grid = grid.clone();
    let mut position = *start_position;

    for &direction in movements {
        let candidate_position = position.step(direction);
        match grid.get(&candidate_position) {
            Some(Tile::Free) => { position = candidate_position; },
            Some(Tile::Wall) | None => {},
            Some(Tile::Box) => {
                if let Some(free_position) = find_free_position(&grid, position, direction) {
                    grid.set(&free_position, Tile::Box);
                    grid.set(&candidate_position, Tile::Free);
                    position = candidate_position;
                }
            }
        }
    }

    grid.into_iter()
        .filter(|(_, t)| *t == Tile::Box)
        .map(|(Position(i, j), _)| i as usize * 100 + j as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    const EXAMPLE2: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(10092, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(2028, part1(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1415498, part1(&parse(include_str!("../input/2024/day15.txt")).unwrap()));
    }
}
