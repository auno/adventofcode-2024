use std::str::FromStr;

use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

enum Signal {
    Constant(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect_vec().as_slice() {
            ["0"] => Ok(Self::Constant(false)),
            ["1"] => Ok(Self::Constant(true)),
            [i1, "AND", i2] => Ok(Self::And(i1.to_string(), i2.to_string())),
            [i1, "OR", i2] => Ok(Self::Or(i1.to_string(), i2.to_string())),
            [i1, "XOR", i2] => Ok(Self::Xor(i1.to_string(), i2.to_string())),
            _ => bail!("Unrecognized Signal: {s}"),
        }
    }
}

type Signals = HashMap<String, Signal>;
type Input = Signals;

#[aoc_generator(day24)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let Some((s, v)) = line.split_once(": ") {
                return Ok((s.to_string(), v.parse()?));
            }

            if let Some((v, s)) = line.split_once(" -> ") {
                return Ok((s.to_string(), v.parse()?));
            }

            bail!("Unable to parse input: {line}");
        })
        .collect()
}

fn resolve_signal(signal_values: &mut HashMap<String, bool>, signals: &Signals, signal_name: &str) -> Option<bool> {
    if let Some(value) = signal_values.get(signal_name) {
        return Some(*value);
    }

    let signal = signals.get(signal_name)?;

    let value = match signal {
        Signal::Constant(value) => *value,
        Signal::And(a, b) => {
            let a = resolve_signal(signal_values, signals, a)?;
            let b = resolve_signal(signal_values, signals, b)?;

            a && b
        },
        Signal::Or(a, b) => {
            let a = resolve_signal(signal_values, signals, a)?;
            let b = resolve_signal(signal_values, signals, b)?;

            a || b
        },
        Signal::Xor(a, b) => {
            let a = resolve_signal(signal_values, signals, a)?;
            let b = resolve_signal(signal_values, signals, b)?;

            (a && !b) || (!a && b)
        },
    };

    signal_values.insert(signal_name.to_string(), value);

    Some(value)
}

#[aoc(day24, part1)]
fn part1(signals: &Input) -> u64 {
    let mut signal_values = HashMap::new();

    signals
        .keys()
        .filter(|signal_name| signal_name.starts_with("z"))
        .sorted()
        .rev()
        .filter_map(|signal_name| resolve_signal(&mut signal_values, signals, signal_name))
        .fold(0, |acc, v| (acc << 1) + v as u64)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const EXAMPLE2: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(4, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(2024, part1(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(55920211035878, part1(&parse(include_str!("../input/2024/day24.txt")).unwrap()));
    }
}
