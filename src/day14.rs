use std::cmp::max;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Input = Vec<((isize, isize), (isize, isize))>;
type NormalizedInput = Vec<((usize, usize), (usize, usize))>;

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

fn normalize(input: &Input, (height, width): (usize, usize)) -> NormalizedInput {
    input
        .iter()
        .map(|((i, j), (vi, vj))| (
            (i.rem_euclid(height as isize) as usize, j.rem_euclid(width as isize) as usize),
            (vi.rem_euclid(height as isize) as usize, vj.rem_euclid(width as isize) as usize),
        ))
        .collect_vec()
}

fn simulate(input: &NormalizedInput, (height, width): (usize, usize), iterations: usize) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    input
        .iter()
        .map(move |((i, j), (vi, vj))| (
            (i + vi * iterations) % height,
            (j + vj * iterations) % width,
        ))
}

fn part1_with_dimensions(input: &Input, (height, width): (usize, usize)) -> Option<usize> {
    let input = normalize(input, (height, width));

    simulate(&input, (height, width), 100)
        .filter(|(i, j)| i % (height / 2 + 1) != height / 2 && j % (width / 2 + 1) != width / 2)
        .map(|(i, j)| (i / (height / 2 + 1), (j / (width / 2 + 1))))
        .sorted()
        .dedup_with_count()
        .map(|(c, _)| c)
        .reduce(|a, b| a * b)
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> Option<usize> {
    part1_with_dimensions(input, (103, 101))
}

fn variance(values: &[usize]) -> usize {
    let mean = values.iter().sum::<usize>() / values.len();
    values.iter().map(|value| value.abs_diff(mean).pow(2)).sum::<usize>() / values.len()
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> Option<usize> {
    let (height, width) = (103, 101);
    let input = normalize(input, (height, width));

    let (variances_i, variances_j): (Vec<_>, Vec<_>) = (0..max(height, width))
        .map(|iterations| {
            let (is, js): (Vec<_>, Vec<_>) = simulate(&input, (height, width), iterations).unzip();
            ((iterations, variance(&is)), (iterations, variance(&js)))
        })
        .unzip();
    let (offset_i, _) = *variances_i.iter().min_by_key(|(_, v)| v).unwrap();
    let (offset_j, _) = *variances_j.iter().min_by_key(|(_, v)| v).unwrap();

    (0..(height * width))
        .find(|&iterations| iterations.abs_diff(offset_i) % height == 0 && iterations.abs_diff(offset_j) % width == 0)
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

    #[test]
    fn part2_input() {
        assert_eq!(Some(8179), part2(&parse(include_str!("../input/2024/day14.txt")).unwrap()));
    }

}
