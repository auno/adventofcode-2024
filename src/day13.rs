use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;

type ClawMachine = ((u32, u32), (u32, u32), (u32, u32));
type Input = Vec<ClawMachine>;

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Input> {
    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let (ax, ay) = scan_fmt!(lines.next().context("Unexpected end of input")?, "Button A: X+{d}, Y+{d}", u32, u32)?;
            let (bx, by) = scan_fmt!(lines.next().context("Unexpected end of input")?, "Button B: X+{d}, Y+{d}", u32, u32)?;
            let (px, py) = scan_fmt!(lines.next().context("Unexpected end of input")?, "Prize: X={d}, Y={d}", u32, u32)?;

            Ok(((ax, ay), (bx, by), (px, py)))
        })
        .collect()
}

fn find_cheapest_prize(machine: &ClawMachine) -> Option<u32> {
    let ((ax, ay), (bx, by), (px, py)) = *machine;

    (0..=100)
        .filter_map(|a| {
            let x = px.checked_sub(a * ax)?;
            let y = py.checked_sub(a * ay)?;

            if x % bx != 0 || y % by != 0 || x / bx != y / by {
                return None;
            }

            let b = x / bx;

            Some(a * 3 + b)
        })
        .min()
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(find_cheapest_prize)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn find_cheapest_prize1() {
        assert_eq!(3, find_cheapest_prize(&((8, 12), (2, 3), (8, 12))).unwrap())
    }

    #[test]
    fn part1_example1() {
        assert_eq!(480, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        let ans = part1(&parse(include_str!("../input/2024/day13.txt")).unwrap());
        assert!(56126 > ans);
        assert!(34773 < ans);
        assert!(47793 > ans);
        assert_eq!(36954, ans);
    }
}
