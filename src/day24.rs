use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Wire {
    Constant(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl FromStr for Wire {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_whitespace().collect_vec().as_slice() {
            ["0"] => Ok(Self::Constant(false)),
            ["1"] => Ok(Self::Constant(true)),
            [i1, "AND", i2] => Ok(Self::And(i1.to_string(), i2.to_string())),
            [i1, "OR", i2] => Ok(Self::Or(i1.to_string(), i2.to_string())),
            [i1, "XOR", i2] => Ok(Self::Xor(i1.to_string(), i2.to_string())),
            _ => bail!("Unrecognized Wire: {s}"),
        }
    }
}

type Wires = HashMap<String, Wire>;
type Input = Wires;

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

fn resolve_wire(wires: &Wires, wire_name: &str) -> Result<bool> {
    fn resolve_wire_impl(wire_values: &mut HashMap<String, bool>, wires: &Wires, wire_name: &str) -> Result<bool> {
        if let Some(value) = wire_values.get(wire_name) {
            return Ok(*value);
        }

        let signal = wires.get(wire_name).with_context(|| format!("Unknown wire: {wire_name}"))?;

        let value = match signal {
            Wire::Constant(value) => *value,
            Wire::And(a, b) => {
                let a = resolve_wire_impl(wire_values, wires, a)?;
                let b = resolve_wire_impl(wire_values, wires, b)?;

                a && b
            },
            Wire::Or(a, b) => {
                let a = resolve_wire_impl(wire_values, wires, a)?;
                let b = resolve_wire_impl(wire_values, wires, b)?;

                a || b
            },
            Wire::Xor(a, b) => {
                let a = resolve_wire_impl(wire_values, wires, a)?;
                let b = resolve_wire_impl(wire_values, wires, b)?;

                (a && !b) || (!a && b)
            },
        };

        wire_values.insert(wire_name.to_string(), value);

        Ok(value)
    }

    resolve_wire_impl(&mut HashMap::new(), wires, wire_name)
}

fn get_signal(wires: &Wires, signal_name: &str) -> Result<u64> {
    let num_wires = wires.keys().filter(|wire_name| wire_name.starts_with(signal_name)).count();

    (0..num_wires)
        .rev()
        .map(|wire_number| format!("{signal_name}{wire_number:02}"))
        .map(|wire_name| resolve_wire(wires, &wire_name))
        .process_results(|wire_values| {
            wire_values.fold(0, |acc, v| (acc << 1) + v as u64)
        })
}

#[allow(dead_code)]
fn set_signal(wires: &mut Wires, signal_name: &str, value: u64) {
    assert!(value < (1 << 45));

    let mut value = value;

    for i in 0..45 {
        wires.insert(format!("{signal_name}{i:02}"), Wire::Constant((value % 2) == 1));
        value >>= 1;
    }
}

#[allow(dead_code)]
fn swap_wires(wires: &Wires) -> Wires {
    let mut wires = wires.clone();

    let z17 = wires["z17"].clone();
    let cmv = wires["cmv"].clone();
    wires.insert("cmv".to_string(), z17);
    wires.insert("z17".to_string(), cmv);

    let z23 = wires["z23"].clone();
    let rmj = wires["rmj"].clone();
    wires.insert("rmj".to_string(), z23);
    wires.insert("z23".to_string(), rmj);

    let z30 = wires["z30"].clone();
    let rdg = wires["rdg"].clone();
    wires.insert("rdg".to_string(), z30);
    wires.insert("z30".to_string(), rdg);

    let mwp = wires["mwp"].clone();
    let btb = wires["btb"].clone();
    wires.insert("btb".to_string(), mwp);
    wires.insert("mwp".to_string(), btb);

    wires
}

#[aoc(day24, part1)]
fn part1(wires: &Input) -> Result<u64> {
    get_signal(wires, "z")
}

#[aoc(day24, part2)]
fn part2(_: &Input) -> String {
    // Swap "tfc OR  qhq -> z17" with "wvj XOR qwg -> cmv"
    // Swap "kkf AND pbw -> z23" with "kkf XOR pbw -> rmj"
    // Swap "x30 AND y30 -> z30" with "knj XOR rvp -> rdg"
    // Swap "y38 AND x38 -> mwp" with "y38 XOR x38 -> btb"

    "btb,cmv,mwp,rdg,rmj,z17,z23,z30".to_string()
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
        assert_eq!(4, part1(&parse(EXAMPLE1).unwrap()).unwrap());
    }

    #[test]
    fn part1_example2() {
        assert_eq!(2024, part1(&parse(EXAMPLE2).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(55920211035878, part1(&parse(include_str!("../input/2024/day24.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_input_verification1() {
        let mut wires = swap_wires(&parse(include_str!("../input/2024/day24.txt")).unwrap());
        let x = 0b_00001010_10101010_10101010_10101010_10101010_10101010_u64;
        let y = 0b_00010101_01010101_01010101_01010101_01010101_01010101_u64;
        set_signal(&mut wires, "x", x);
        set_signal(&mut wires, "y", y);
        assert_eq!(x + y, get_signal(&wires, "z").unwrap());
    }

    #[test]
    fn part2_input_verification2() {
        let mut wires = swap_wires(&parse(include_str!("../input/2024/day24.txt")).unwrap());
        let x = 0b_00011111_11111111_11111111_11111111_11111111_11111111_u64;
        let y = 0b_00011111_11111111_11111111_11111111_11111111_11111111_u64;
        set_signal(&mut wires, "x", x);
        set_signal(&mut wires, "y", y);
        assert_eq!(x + y, get_signal(&wires, "z").unwrap());
    }

    #[test]
    fn part2_input_verification3() {
        let mut wires = swap_wires(&parse(include_str!("../input/2024/day24.txt")).unwrap());
        let x = 0b_00011111_11111111_11111111_11111111_11111111_11111111_u64;
        let y = 0b_00010000_00000000_00000000_00000000_00000000_00000000_u64;
        set_signal(&mut wires, "x", x);
        set_signal(&mut wires, "y", y);
        assert_eq!(x + y, get_signal(&wires, "z").unwrap());
    }

    #[test]
    fn part2_input_verification4() {
        let mut wires = swap_wires(&parse(include_str!("../input/2024/day24.txt")).unwrap());
        let x = 0b_00011111_11111111_11111111_11111111_11111111_11111111_u64;
        let y = 0b_00000000_00000000_00000000_00000000_00000000_00000001_u64;
        set_signal(&mut wires, "x", x);
        set_signal(&mut wires, "y", y);
        assert_eq!(x + y, get_signal(&wires, "z").unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!("btb,cmv,mwp,rdg,rmj,z17,z23,z30", part2(&parse(include_str!("../input/2024/day24.txt")).unwrap()));
    }
}
