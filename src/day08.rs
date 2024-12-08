use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna(char);

impl TryFrom<char> for Antenna {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'a'..='z' | 'A'..='Z' | '0'..='9' => Ok(Antenna(value)),
            _ => bail!("Could not parse Antenna: '{value}'"),
        }
    }
}

type Input = (Vec<(Antenna, (isize, isize))>, (isize, isize));

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Input> {
    let rows = input.lines().count() as isize;
    let cols = input.lines().next().map(str::len).unwrap_or(0) as isize;

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .filter(|(_, c)| *c != '.')
        .map(move |((i, j), c)| Ok((Antenna::try_from(c)?, (i as isize, j as isize))))
        .collect::<Result<_>>();

    Ok((antennas?, (rows, cols)))
}

fn solve<K>(
    antennas: &[(Antenna, (isize, isize))],
    (rows, cols): (isize, isize),
    antinode_coefficients: K
) -> usize
    where K: IntoIterator<Item = isize> + Clone
{
    antennas
        .iter()
        .copied()
        .into_grouping_map()
        .collect::<Vec<_>>()
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| [(a, b), (b, a)])
                .flat_map(|((ai, aj), (bi, bj))| {
                    antinode_coefficients
                        .clone()
                        .into_iter()
                        .map(move |k| (bi + k * (bi - ai), bj + k * (bj - aj)))
                        .take_while(|(i, j)| (0..rows).contains(i) && (0..cols).contains(j))
                })
        })
        .unique()
        .count()
}

#[aoc(day8, part1)]
fn part1((antennas, dimensions): &Input) -> usize {
    solve(antennas, *dimensions, 1..2)
}

#[aoc(day8, part2)]
fn part2((antennas, dimensions): &Input) -> usize {
    solve(antennas, *dimensions, 0..)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(14, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(323, part1(&parse(include_str!("../input/2024/day8.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(34, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1077, part2(&parse(include_str!("../input/2024/day8.txt")).unwrap()));
    }
}
