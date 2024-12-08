use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::{chain, Itertools};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Tile {
    #[default]
    Free,
    Obstructed,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Tile::Free),
            '#' => Ok(Tile::Obstructed),
            _ => bail!("Unknown Tile: {value}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Position(isize, isize);

impl Position {
    fn step(self, dir: Direction) -> Position {
        let Position(i, j) = self;
        match dir {
            Direction::Up => Position(i - 1, j),
            Direction::Down => Position(i + 1, j),
            Direction::Left => Position(i, j - 1),
            Direction::Right => Position(i, j + 1),
        }
    }
}

#[derive(Clone)]
struct Grid<T> where T: Copy + Clone {
    store: Vec<Vec<T>>,
    rows: isize,
    cols: isize,
}

impl<T> Grid<T> where T: Copy + Clone {
    fn new_with_value(rows: isize, cols: isize, value: T) -> Grid<T> {
        Grid {
            store: vec![vec![value; cols as usize]; rows as usize],
            rows,
            cols,
        }
    }
}

impl<T> Grid<T> where T: Default + Copy + Clone {
    fn new(rows: isize, cols: isize) -> Grid<T> {
        Self::new_with_value(rows, cols, T::default())
    }

    fn get(&self, &Position(i, j): &Position) -> Option<&T> {
        if i < 0 || i >= self.rows || j < 0 || j >= self.cols {
            return None;
        }

        Some(&self.store[i as usize][j as usize])
    }

    fn get_mut(&mut self, &Position(i, j): &Position) -> Option<&mut T> {
        if i < 0 || i >= self.rows || j < 0 || j >= self.cols {
            return None;
        }

        Some(&mut self.store[i as usize][j as usize])
    }

    fn set(&mut self, &Position(i, j): &Position, value: T) {
        self.store[i as usize][j as usize] = value;
    }
}

type Map = Grid<Tile>;

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<(Map, Position)> {
    let rows = input.lines().count() as isize;
    let cols = input.lines().next().map(str::len).unwrap_or(0) as isize;

    let (map, guard_pos) = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line
            .chars()
            .enumerate()
            .map(move |(j, c)| (Position(i as isize, j as isize), c))
        )
        .map(|(pos, c)| -> Result<(Position, Tile, bool)> {
            match c {
                '^' => Ok((pos, Tile::Free, true)),
                _ => Ok((pos, Tile::try_from(c)?, false)),
            }
        })
        .fold_ok((Grid::new(rows, cols), None), |(mut acc, guard_pos), (pos, tile, is_guard_pos)| {
            acc.set(&pos, tile);
            (acc, if is_guard_pos { Some(pos) } else { guard_pos })
        })?;

    Ok((map, guard_pos.context("No guard position found")?))
}

fn resolve_path(map: &Map, position: Position) -> (Vec<Position>, bool) {
    let mut position = position;
    let mut direction = Direction::Up;
    let mut path = vec![position];
    let mut seen = Grid::new_with_value(map.rows, map.cols, [false; 4]);

    loop {
        let candidate_position = position.step(direction);

        if seen.get(&candidate_position).unwrap_or(&[false; 4])[direction as usize] {
            return (path, true);
        }

        match map.get(&candidate_position) {
            Some(Tile::Free) => {
                position = candidate_position;
                seen.get_mut(&candidate_position).unwrap()[direction as usize] = true;
                path.push(candidate_position);
            },
            Some(Tile::Obstructed) => {
                direction = direction.turn();
            },
            None => break,
        }
    }

    (path, false)
}

struct JumpMap {
    map: Grid<[Option<Position>; 4]>,
}

impl JumpMap {
    fn new(map: &Map) -> JumpMap {
        let mut jump_map = Grid::new_with_value(map.rows, map.cols, [None; 4]);

        let rows = map.rows;
        let cols = map.cols;

        let starters = Vec::from_iter(chain!(
            (0..rows).map(|i| (Position(i, -1), Direction::Right)),
            (0..rows).map(|i| (Position(i, cols), Direction::Left)),
            (0..cols).map(|j| (Position(-1, j), Direction::Down)),
            (0..cols).map(|j| (Position(rows, j), Direction::Up)),
        ));

        'outer: for (position, direction) in starters {
            let mut current_position = position;
            let mut sources = vec![];

            loop {
                let next_position = current_position.step(direction);

                match map.get(&next_position) {
                    None => { continue 'outer; },
                    Some(Tile::Free) => {},
                    Some(Tile::Obstructed) => {
                        for source in &sources {
                            jump_map.get_mut(source).unwrap()[direction as usize] = Some(current_position);
                        }
                        sources.clear();
                    },
                }

                current_position = next_position;
                sources.push(current_position);
            }
        }

        JumpMap { map: jump_map }
    }

    fn jump(&self, current_position: Position, current_direction: Direction, obstruction_position: Position) -> Option<Position> {
        let Position(ci, cj) = current_position;
        let Position(oi, oj) = obstruction_position;

        let jump_destination = self.map
            .get(&current_position)
            .and_then(|dv| dv[current_direction as usize]);

        /* If jump_destination is None, use out-of-bounds coordinates to satisfy interception check. */
        let Position(di, dj) = jump_destination.unwrap_or(match current_direction {
            Direction::Up | Direction::Left => Position(-1, -1),
            Direction::Down | Direction::Right => Position(1000, 1000),
        });

        match current_direction {
            Direction::Left if ci != oi || cj < oj || oj < dj => jump_destination,
            Direction::Right if ci != oi || cj > oj || oj > dj => jump_destination,
            Direction::Up if cj != oj || ci < oi || oi < di => jump_destination,
            Direction::Down if cj != oj || ci > oi || oi > di => jump_destination,
            _ => Some(obstruction_position.step(current_direction.opposite())),
        }
    }
}

#[aoc(day6, part1)]
fn part1((map, guard_pos): &(Map, Position)) -> usize {
    let (path, _) = resolve_path(map, *guard_pos);
    path.iter().unique().count()
}

#[aoc(day6, part2)]
fn part2((map, guard_pos): &(Map, Position)) -> usize {
    let (path, _) = resolve_path(map, *guard_pos);
    let mut map = map.clone();
    let jump_map = JumpMap::new(&map);

    path.into_iter()
        .unique()
        .filter(|position| position != guard_pos)
        .filter(|obstruction_position| {
            map.set(obstruction_position, Tile::Obstructed);

            let mut current_position = *guard_pos;
            let mut current_direction = Direction::Up;
            let mut seen = Grid::new_with_value(map.rows, map.cols, [false; 4]);

            let cycle = loop {
                seen.get_mut(&current_position).unwrap()[current_direction as usize] = true;

                let Some(next_position) = jump_map.jump(current_position, current_direction, *obstruction_position) else {
                    break false;
                };
                let next_direction = current_direction.turn();

                if seen.get(&next_position).unwrap_or(&[false; 4])[next_direction as usize] {
                    break true;
                }

                current_position = next_position;
                current_direction = next_direction;
            };

            map.set(obstruction_position, Tile::Free);

            cycle
        })
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(41, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(5095, part1(&parse(include_str!("../input/2024/day6.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(6, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1933, part2(&parse(include_str!("../input/2024/day6.txt")).unwrap()));
    }
}
