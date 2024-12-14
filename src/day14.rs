use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Input = Vec<((isize, isize), (isize, isize))>;

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let (j, i, vj, vi) = scan_fmt!(line, "p={d},{d} v={d},{d}", isize, isize, isize, isize)?;
            Ok(((i, j), (vi, vj)))
        })
        .collect()
}

fn part1_with_dimensions(input: &Input, (height, width): (isize, isize)) -> Option<usize> {
    input
        .iter()
        .map(|((i, j), (vi, vj))| (
            (i + vi * 100).rem_euclid(height),
            (j + vj * 100).rem_euclid(width),
        ))
        .filter(|(i, j)| i % (height / 2 + 1) != height / 2 && j % (width / 2 + 1) != width / 2)
        .map(|(i, j)| (i / (height / 2 + 1), (j / (width / 2 + 1))))
        .counts()
        .values()
        .copied()
        .reduce(|a, b| a * b)
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> Option<usize> {
    part1_with_dimensions(input, (103, 101))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(Some(12), part1_with_dimensions(&parse(EXAMPLE1).unwrap(), (7, 11)));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(232253028), part1(&parse(include_str!("../input/2024/day14.txt")).unwrap()));
    }
}
