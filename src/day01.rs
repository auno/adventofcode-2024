use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    Ok(
        input
            .lines()
            .map(|line| {
                let (left, right) = line
                    .split_once("   ")
                    .context(format!("Unable to parse line: {}", line))?;
                let left: u32 = left.parse()?;
                let right: u32 = right.parse()?;

                Ok((left, right)) as Result<(u32, u32)>
            })
            .process_results(|iter| iter.unzip())?
    )
}

#[aoc(day1, part1)]
fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let left = left.iter().sorted();
    let right = right.iter().sorted();

    left.zip(right)
        .map(|(l, r)| {
            l.abs_diff(*r)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn part1_example1() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(11, part1(&parse(input).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2904518, part1(&parse(include_str!("../input/2024/day1.txt")).unwrap()));
    }
}
