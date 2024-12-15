use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{chain, Itertools};

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

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
enum Tile2 {
    #[default]
    Free,
    Wall,
    BoxLeft,
    BoxRight,
}

fn find_pushable_boxes(grid: &Grid<Tile2>, position: Position, direction: Direction) -> Option<Vec<Position>> {
    let positions = match (direction, grid.get(&position)) {
        (Direction::Up | Direction::Down, Some(Tile2::BoxLeft)) => vec![position, position.step(Direction::Right)],
        (Direction::Up | Direction::Down, Some(Tile2::BoxRight)) => vec![position.step(Direction::Left), position],
        (Direction::Left | Direction::Right, Some(Tile2::BoxLeft | Tile2::BoxRight)) => vec![position],
        (_, t @ (Some(Tile2::Free | Tile2::Wall) | None)) => panic!("Position must point to a BoxLeft or BoxRight: {t:?}"),
    };

    let candidate_positions = positions.iter().map(|p| p.step(direction)).collect_vec();
    let candidate_tiles = candidate_positions
        .iter()
        .map(|p| grid.get(p))
        .collect::<Option<Vec<_>>>()?;

    match candidate_tiles[..] {
        [Tile2::Wall] => None,
        [Tile2::Wall, _] => None,
        [_, Tile2::Wall] => None,
        [Tile2::Free] => Some(positions),
        [Tile2::Free, Tile2::Free] => Some(positions),
        [Tile2::BoxLeft | Tile2::BoxRight] => Some(chain!(positions, find_pushable_boxes(grid, candidate_positions[0], direction)?).collect_vec()),
        [Tile2::BoxRight, Tile2::Free] => Some(chain!(positions, find_pushable_boxes(grid, candidate_positions[0], direction)?).collect_vec()),
        [Tile2::Free, Tile2::BoxLeft] => Some(chain!(positions, find_pushable_boxes(grid, candidate_positions[1], direction)?).collect_vec()),
        [Tile2::BoxLeft, Tile2::BoxRight] => Some(chain!(positions, find_pushable_boxes(grid, candidate_positions[0], direction)?).collect_vec()),
        [Tile2::BoxRight, Tile2::BoxLeft] => Some(chain!(
            positions,
            find_pushable_boxes(grid, candidate_positions[0], direction)?,
            find_pushable_boxes(grid, candidate_positions[1], direction)?,
        ).collect_vec()),
        _ => unreachable!("Unexpected candidates: {candidate_tiles:?}"),
    }
}

#[aoc(day15, part2)]
fn part2((original_grid, start_position, movements): &Input) -> usize {
    let mut grid = Grid::<Tile2>::new(original_grid.rows(), original_grid.cols::<isize>() * 2);
    let mut position = Position(start_position.0, start_position.1 * 2);

    for (Position(i, j), ot) in original_grid {
        let (t1, t2) = match ot {
            Tile::Free => (Tile2::Free, Tile2::Free),
            Tile::Box => (Tile2::BoxLeft, Tile2::BoxRight),
            Tile::Wall => (Tile2::Wall, Tile2::Wall),
        };

        grid.set(&Position(i, j * 2), t1);
        grid.set(&Position(i, j * 2 + 1), t2);
    }

    for &direction in movements {
        let candidate_position = position.step(direction);
        match grid.get(&candidate_position) {
            Some(Tile2::Free) => { position = candidate_position; },
            Some(Tile2::Wall) | None => {},
            Some(Tile2::BoxLeft | Tile2::BoxRight) => {
                if let Some(positions) = find_pushable_boxes(&grid, candidate_position, direction) {
                    let boxes = positions
                        .iter()
                        .unique()
                        .map(|p| (*p, *grid.get(p).unwrap()))
                        .collect_vec();

                    for (p, _) in &boxes {
                        grid.set(p, Tile2::Free);
                    }

                    for &(p, t) in &boxes {
                        grid.set(&p.step(direction), t);
                    }

                    position = candidate_position;
                }
            }
        }
    }

    grid.into_iter()
        .filter(|(_, t)| *t == Tile2::BoxLeft)
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

    const EXAMPLE3: &str = indoc! {"
        #######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^
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

    #[test]
    fn part2_example1() {
        assert_eq!(9021, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(105 + 207 + 306, part2(&parse(EXAMPLE3).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1432898, part2(&parse(include_str!("../input/2024/day15.txt")).unwrap()));
    }
}
