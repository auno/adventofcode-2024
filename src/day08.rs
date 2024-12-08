
use std::collections::HashMap;

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

type Input = (HashMap<(isize, isize), Antenna>, (isize, isize));

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Input> {
    let rows = input.lines().count() as isize;
    let cols = input.lines().next().map(str::len).unwrap_or(0) as isize;

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .filter(|(_, c)| *c != '.')
        .map(move |((i, j), c)| Ok(((i as isize, j as isize), Antenna::try_from(c)?)))
        .collect::<Result<_>>();

    Ok((antennas?, (rows, cols)))
}

#[aoc(day8, part1)]
fn part1((antennas, (rows, cols)): &Input) -> usize {
    antennas
        .values()
        .unique()
        .flat_map(|&frequency| {
            antennas
                .iter()
                .filter(move |(_, &a)| a == frequency)
                .map(|(p, _)| *p)
                .tuple_combinations()
                .flat_map(|(a, b)| [(a, b), (b, a)])
                .map(|((ai, aj), (bi, bj))| (
                    bi + (bi - ai),
                    bj + (bj - aj),
                ))
        })
        .unique()
        .filter(|(i, j)| (0..*rows).contains(i) && (0..*cols).contains(j))
        .count()
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
}
