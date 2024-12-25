use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = (Vec<[u8; 5]>, Vec<[u8; 5]>);

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Input> {
    let mut locks = vec![];
    let mut keys = vec![];

    for chunk in input.split("\n\n") {
        let mut code = [0; 5];

        for line in chunk.lines() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    code[i] += 1;
                }
            }
        }

        for c in &mut code {
            *c -= 1;
        }

        if chunk.starts_with('#') {
            locks.push(code);
        } else {
            keys.push(code);
        }
    }

    Ok((locks, keys))
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
