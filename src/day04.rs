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

fn resolve_candidates(
    position: (isize, isize),
    valid_patterns: &[Vec<((isize, isize), char)>],
) -> impl Iterator<Item = Vec<((isize, isize), char)>> + use<'_> {
    valid_patterns
        .iter()
        .map(move |valid_pattern| {
            valid_pattern
                .iter()
                .copied()
                .map(|(offset, valid_char)| ((position.0 + offset.0, position.1 + offset.1), valid_char))
                .collect_vec()
        })
}

fn is_valid_candiate(input: &HashMap<(isize, isize), char>, candidate: &[((isize, isize), char)]) -> bool {
    candidate
        .iter()
        .all(|(position, expected_char)| {
            input
                .get(position)
                .map_or(false, |found_char| found_char == expected_char)
        })
}

fn resolve_candidate_position(position: &(isize, isize), found_char: char, expected_char: char) -> Option<(isize, isize)> {
    if found_char == expected_char {
        return Some(*position);
    }

    None
}

fn count_valid_positions(
    input: &HashMap<(isize, isize), char>,
    valid_patterns: &[Vec<((isize, isize), char)>],
    candidate_position_char: char,
) -> usize {
    input
        .iter()
        .filter_map(|(position, c)| resolve_candidate_position(position, *c, candidate_position_char))
        .flat_map(|candidate_position| resolve_candidates(candidate_position, valid_patterns))
        .filter(|candidate| is_valid_candiate(input, candidate))
        .count()
}

#[aoc(day4, part1)]
fn part1(input: &HashMap<(isize, isize), char>) -> usize {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    let valid_possibilities =
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
            .into_iter()
            .map(|(i, j): (isize, isize)| {
                XMAS.iter()
                    .enumerate()
                    .skip(1) /* No need to check X */
                    .rev() /* Significantly faster due border positions and input characteristicts */
                    .map(|(offset, c)| (offset as isize, *c))
                    .map(|(offset, c)| ((i * offset, j * offset), c))
                    .collect_vec()
            })
            .collect_vec();
    count_valid_positions(input, &valid_possibilities, XMAS[0])
}

#[aoc(day4, part2)]
fn part2(input: &HashMap<(isize, isize), char>) -> usize {
    let valid_patterns = [
        vec![((-1, -1), 'M'), ((1, 1), 'S'), ((-1, 1), 'M'), ((1, -1), 'S')],
        vec![((-1, -1), 'S'), ((1, 1), 'M'), ((-1, 1), 'M'), ((1, -1), 'S')],
        vec![((-1, -1), 'M'), ((1, 1), 'S'), ((-1, 1), 'S'), ((1, -1), 'M')],
        vec![((-1, -1), 'S'), ((1, 1), 'M'), ((-1, 1), 'S'), ((1, -1), 'M')],
    ];

    count_valid_positions(input, &valid_patterns, 'A')
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
