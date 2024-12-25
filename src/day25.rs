use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = (Vec<[u8; 5]>, Vec<[u8; 5]>);

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Input> {
    input
        .split("\n\n")
        .try_fold((vec![], vec![]), |(mut locks, mut keys), chunk| {
            let code = chunk
                .lines()
                .skip(1)
                .take(5)
                .try_fold([0; 5], |code, line| {
                    code.iter().zip(line.chars())
                        .map(|(a, b)| a + (b == '#') as u8)
                        .collect_vec()
                        .try_into()
                        .map_err(|_| anyhow!("Unable to parse line, wrong length: {line}"))
                })?;

            if chunk.starts_with('#') {
                locks.push(code);
            } else {
                keys.push(code);
            }

            Ok((locks, keys))
        })
}

#[aoc(day25, part1)]
fn part1((locks, keys): &Input) -> usize {
    locks.iter().cartesian_product(keys)
            .filter(|(lock, key)| {
                lock.iter()
                    .zip(key.iter())
                    .all(|(l, k)| l + k <= 5)
            })
            .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(3, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(3196, part1(&parse(include_str!("../input/2024/day25.txt")).unwrap()));
    }
}
