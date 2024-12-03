use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> Result<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    pattern
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a = a.parse::<u32>()?;
            let b = b.parse::<u32>()?;

            Ok(a * b)
        })
        .sum()
}


#[aoc(day3, part2)]
fn part2(input: &str) -> Result<u32> {
    let ins_pattern = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let (_, sum) = ins_pattern
        .captures_iter(input)
        .fold((true, 0), |(mul_enabled, sum), c| {
            match c.extract() {
                (_, ["do()"]) => (true, sum),
                (_, ["don't()"]) => (false, sum),
                (_, [mul]) => {
                    if !mul_enabled {
                        return (mul_enabled, sum)
                    }

                    let mul = mul_pattern.captures(mul).unwrap();
                    let (_, [a, b]) = mul.extract();
                    let a = a.parse::<u32>().unwrap();
                    let b = b.parse::<u32>().unwrap();
                    (mul_enabled, sum + a * b)
                },
            }
        });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example1() {
        assert_eq!(161, part1(&parse(EXAMPLE1)).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(184576302, part1(&parse(include_str!("../input/2024/day3.txt"))).unwrap());
    }

    #[test]
    fn part2_example1() {
        let example1 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part2(&parse(example1)).unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!(118173507, part2(&parse(include_str!("../input/2024/day3.txt"))).unwrap());
    }
}
