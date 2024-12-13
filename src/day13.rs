use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;

type ClawMachine = ((i64, i64), (i64, i64), (i64, i64));
type Input = Vec<ClawMachine>;

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Input> {
    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let (ax, ay) = scan_fmt!(lines.next().context("Unexpected end of input")?, "Button A: X+{d}, Y+{d}", i64, i64)?;
            let (bx, by) = scan_fmt!(lines.next().context("Unexpected end of input")?, "Button B: X+{d}, Y+{d}", i64, i64)?;
            let (px, py) = scan_fmt!(lines.next().context("Unexpected end of input")?, "Prize: X={d}, Y={d}", i64, i64)?;

            Ok(((ax, ay), (bx, by), (px, py)))
        })
        .collect()
}

fn find_cost(machine: ClawMachine) -> Option<i64> {
    let ((ax, ay), (bx, by), (px, py)) = machine;

    let det = ax * by - ay * bx;
    let a = (by * px - bx * py) / det;
    let b = (ax * py - ay * px) / det;

    if (ax * a + bx * b, ay * a + by * b) != (px, py) {
        return None;
    }

    Some(a * 3 + b)
}

fn find_costs(machines: impl IntoIterator<Item = ClawMachine>) -> impl IntoIterator<Item = Option<i64>> {
    machines
        .into_iter()
        .map(find_cost)
}

fn shift_targets(machines: &Input, offset: i64) -> impl IntoIterator<Item = ClawMachine> + use<'_> {
    machines
        .iter()
        .map(move |&(a, b, (px, py))| (a, b, (px + offset, py + offset)))
}

fn solve(machines: &Input, multiplier: i64) -> i64 {
    find_costs(shift_targets(machines, multiplier)).into_iter().flatten().sum()
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> i64 {
    solve(input, 0)
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> i64 {
    solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;

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

    #[test]
    fn part2_example1() {
        let machines = parse(EXAMPLE1).unwrap();
        let has_solutions = find_costs(shift_targets(&machines, 10000000000000))
            .into_iter()
            .map(|solution| solution.is_some())
            .collect_vec();
        assert_eq!(vec![false, true, false, true], has_solutions);
    }

    #[test]
    fn part2_input() {
        assert_eq!(79352015273424, part2(&parse(include_str!("../input/2024/day13.txt")).unwrap()));
    }
}
