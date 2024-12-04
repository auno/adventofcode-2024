use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

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
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

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

#[aoc(day4, part2)]
fn part2(input: &HashMap<(isize, isize), char>) -> usize {
    let a_positions = input
        .iter()
        .filter(|(_, c)| **c == 'A')
        .map(|(pos, _)| *pos)
        .collect_vec();

    let valid_possibilities = [
        [((-1, -1), 'M'), ((1, 1), 'S'), ((-1, 1), 'M'), ((1, -1), 'S'), ((0, 0), 'A')],
        [((-1, -1), 'S'), ((1, 1), 'M'), ((-1, 1), 'M'), ((1, -1), 'S'), ((0, 0), 'A')],
        [((-1, -1), 'M'), ((1, 1), 'S'), ((-1, 1), 'S'), ((1, -1), 'M'), ((0, 0), 'A')],
        [((-1, -1), 'S'), ((1, 1), 'M'), ((-1, 1), 'S'), ((1, -1), 'M'), ((0, 0), 'A')],
    ];

    a_positions
        .into_iter()
        .filter(|pos| {
            valid_possibilities
                .into_iter()
                .any(|valid_possibility| {
                    valid_possibility
                        .iter()
                        .all(|(offset, valid_char)| {
                            input
                                .get(&(pos.0 + offset.0, pos.1 + offset.1))
                                .map_or(false, |found_char| found_char == valid_char)
                        })
                })
        })
        .count()
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

    #[test]
    fn part2_example1() {
        assert_eq!(9, part2(&parse(EXAMPLE1)));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1890, part2(&parse(include_str!("../input/2024/day4.txt"))));
    }
}
