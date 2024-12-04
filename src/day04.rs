use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

#[aoc_generator(day4)]
fn parse(input: &str) -> HashMap<(isize, isize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as isize, j as isize), c))
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &HashMap<(isize, isize), char>) -> usize {
    let xs = input
        .iter()
        .filter(|(_, c)| **c == XMAS[0])
        .map(|(pos, _)| *pos)
        .collect_vec();

    xs.iter()
        .map(|pos| {
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
                .into_iter()
                .filter(|dir| {
                    let found = (0..4)
                        .map(|offset| input
                        .get(&(pos.0 + dir.0 * offset, pos.1 + dir.1 * offset)))
                        .filter(Option::is_some)
                        .map(Option::unwrap);

                    found.eq(XMAS.iter())
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(18, part1(&parse(EXAMPLE1)));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2493, part1(&parse(include_str!("../input/2024/day4.txt"))));
    }
}
