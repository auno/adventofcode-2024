use std::{collections::VecDeque, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
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

#[aoc(day24, part1)]
fn part1(wires: &Input) -> Result<u64> {
    get_signal(wires, "z")
}

fn set_signal(wires: &mut Wires, signal_name: &str, value: u64) {
    assert!(value < (1 << 45));
    // eprintln!("-- 0b{value:045b}   {value:14}");

    let mut value = value;

    for i in 0..45 {
        wires.insert(format!("{signal_name}{i:02}"), Wire::Constant((value % 2) == 1));
        value >>= 1;
    }
}

fn resolve_outputs(reverse: &HashMap<String, Vec<String>>, wire_names: &[String]) -> HashSet<String> {
    let mut queue = VecDeque::from_iter(wire_names);
    let mut outputs = HashSet::from_iter(wire_names.iter().cloned());
    let mut seen = HashSet::new();

    while let Some(wire_name) = queue.pop_back() {
        if !seen.insert(wire_name) { continue; }

        let Some(wires) = reverse.get(wire_name) else { continue; };
        outputs.extend(wires.iter().cloned());
        queue.extend(wires);
    }

    outputs
}

fn resolve_inputs(wires: &Wires, wire_names: &[String]) -> HashSet<String> {
    let mut queue = VecDeque::from_iter(wire_names.iter().cloned());
    let mut inputs = HashSet::new();

    while let Some(wire_name) = queue.pop_back() {
        // if wire_name.starts_with("x") || wire_name.starts_with("y") { continue; }
        if !inputs.insert(wire_name.clone()) { continue; }
        let Some(wire) = wires.get(&wire_name) else { continue; };

        match wire {
            Wire::Constant(_) => {},
            Wire::And(a, b) => {
                queue.push_back(a.to_string());
                queue.push_back(b.to_string());
            },
            Wire::Or(a, b) => {
                queue.push_back(a.to_string());
                queue.push_back(b.to_string());
            },
            Wire::Xor(a, b) => {
                queue.push_back(a.to_string());
                queue.push_back(b.to_string());
            },
        }
    }

    inputs
}

fn validate(wires: Wires, best: usize, level: usize) {
    // if level > 4 { return; }
    // eprintln!("=== validate {level} best: {best}");

    let reverse = wires
        .iter()
        .flat_map(|(k, v)| {
            match v {
                Wire::Constant(a) => vec![],
                Wire::And(a, b) => vec![
                    (a.to_string(), k.to_string()),
                    (b.to_string(), k.to_string()),
                ],
                Wire::Or(a, b) => vec![
                    (a.to_string(), k.to_string()),
                    (b.to_string(), k.to_string()),
                ],
                Wire::Xor(a, b) => vec![
                    (a.to_string(), k.to_string()),
                    (b.to_string(), k.to_string()),
                ],
            }
        })
        .into_group_map();

    for i in (0..45).rev() {
        // dbg!(i);
        let test_cases = [
            (1 << i, 0),
            (0, 1 << i),
            (1 << i, 1 << i),
            ((((1 << 45) - 1) ^ ((1 << i) - 1)), 1 << i),
            (1 << i, (((1 << 45) - 1) ^ ((1 << i) - 1))),
            ((((1 << 45) - 1) ^ ((1 << i) - 1)), (((1 << 45) - 1) ^ ((1 << i) - 1))),
            // (((1 << (i + 1)) - 1), 0),
            // (0, ((1 << (i + 1)) - 1)),
            // (((1 << (i + 1)) - 1), ((1 << (i + 1)) - 1)),
        ];

        if let Some(diff) = test_cases
            .into_iter()
            .filter_map(|(x, y)| {
                let mut wires = wires.clone();
                set_signal(&mut wires, "x", x);
                set_signal(&mut wires, "y", y);
                let z_expected = x + y;
                let z_actual = get_signal(&wires, "z").unwrap();
                // if z_actual != z_expected {
                //     dbg!(i, x, y, z_expected, z_actual);
                //     eprintln!("-- ze: 0b{z_expected:045b}");
                //     eprintln!("-- za: 0b{z_actual:045b}");
                // }

                most_significant_diff(z_expected, z_actual)
            })
            .max()
        {
            let last_valid_output = diff + 1;
            // let valid_outputs = (0..last_valid_output)
            //     .map(|i| format!("z{i:02}"))
            //     .collect_vec();
            // // let valid_inputs = resolve_inputs(&wires, &valid_outputs);
            // let inputs_at_diff = resolve_inputs(&wires, &[format!("z{diff:02}")]);

            let valid_inputs = ((i + 1)..45)
                .flat_map(|i| [format!("x{i:02}"), format!("y{i:02}")])
                .collect_vec();
            let valid_wires = resolve_outputs(&reverse, &valid_inputs);
            let wires_at_i = resolve_outputs(&reverse, &[
                format!("x{i:02}"),
                format!("y{i:02}"),
            ]);

            let candidates = wires_at_i.difference(&valid_wires).collect_vec();

            // dbg!(i, level, diff, last_valid_output, best, valid_wires.len(), wires_at_i.len(), &candidates);

            if i < best {
                eprintln!("=== new best {level} best: {best}");

                for (a, b) in candidates.iter().tuple_combinations() {
                    // eprintln!("--------------------------- {a} {b}");
                    let mut wires = wires.clone();
                    let awire = wires.get(*a).unwrap_or_else(|| panic!("--- unwrap {a}")).clone();
                    let bwire = wires.get(*b).unwrap_or_else(|| panic!("--- unwrap {b}")).clone();
                    wires.insert(a.to_string(), bwire);
                    wires.insert(b.to_string(), awire);

                    if has_cycle(&wires) {
                        continue;
                    }

                    validate(wires, last_valid_output, level + 1);
                }
            }

            break;
        }
    }
}

fn has_cycle(wires: &Wires) -> bool {
    fn has_cycle_impl<'a>(wires: &'a Wires, path: &mut HashSet<&'a str>, cleared: &mut HashSet<&'a str>, wire_name: &'a str) -> bool {
        if !path.insert(wire_name) {
            return true;
        }

        let descendants = match wires.get(wire_name).unwrap() {
            Wire::Constant(_) => vec![],
            Wire::And(a, b) => vec![a, b],
            Wire::Or(a, b) => vec![a, b],
            Wire::Xor(a, b) => vec![a, b],
        };

        if descendants.into_iter().any(|descendant| {
            has_cycle_impl(wires, path, cleared, descendant)
        }) {
            return true;
        }

        path.remove(wire_name);
        cleared.insert(wire_name);

        false
    }

    let mut cleared = HashSet::new();

    for wire_name in wires.keys() {
        if cleared.contains(wire_name.as_str()) { continue; }

        if has_cycle_impl(wires, &mut HashSet::new(), &mut cleared, wire_name) {
            return true;
        }
    }

    false
}

fn least_significant_diff(mut a: u64, mut b: u64) -> Option<usize> {
    if a == b { return None; }

    for i in 0.. {
        if a % 2 != b % 2 {
            return Some(i);
        }

        a >>= 1;
        b >>= 1;
    }

    None
}

fn most_significant_diff(mut a: u64, mut b: u64) -> Option<usize> {
    let mut msd = None;
    let mut i = 0;

    while a != b {
        if a % 2 != b % 2 {
            msd = Some(i);
        }

        a >>= 1;
        b >>= 1;
        i += 1;
    }

    msd
}

#[aoc(day24, part2)]
fn part2(wires: &Input) -> Result<String> {
    // let mut wire_values = HashMap::new();

    let x_wire_names = wires.keys().filter(|wire_name| wire_name.starts_with("x")).sorted().collect_vec();
    let y_wire_names = wires.keys().filter(|wire_name| wire_name.starts_with("y")).sorted().collect_vec();
    let z_wire_names = wires.keys().filter(|wire_name| wire_name.starts_with("z")).sorted().collect_vec();

    dbg!(x_wire_names.len());
    dbg!(y_wire_names.len());
    dbg!(z_wire_names.len());

    // let mut wires = wires.clone();
    // wires.retain(|k, _| !k.starts_with("x") && !k.starts_with("y"));
    // let mut wrong_count = 0;
    // let mut wrong_x_bits = vec![0; 45];
    // let mut wrong_y_bits = vec![0; 45];
    // let mut wrong_z_bits = vec![0; 46];

    // {
    //     for xi in (0..45).rev() {
    //         let x = (1 << 44) >> xi;

    //         for yi in (0..45).rev() {
    //             let y = (1 << 44) >> yi;

    //             let mut wires = wires.clone();

    //             set_signal(&mut wires, "x", x);
    //             set_signal(&mut wires, "y", y);

    //             let z_actual = get_signal(&wires, "z")?;
    //             let z_expected = x + y;

    //             if z_actual != z_expected {
    //                 // eprintln!("x: 0b{x:045b}, y: 0b{y:045b}, za: 0b{z_actual:046b}, ze: 0b{z_expected:046b}");
    //                 wrong_count += 1;

    //                 wrong_x_bits[xi] += 1;
    //                 wrong_y_bits[yi] += 1;

    //                 for i in 0..46 {
    //                     if (z_expected >> i) % 2 != (z_actual >> i) {
    //                         wrong_z_bits[i] += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // let xavg = wrong_x_bits.iter().sum::<usize>() / wrong_x_bits.len();
    // let yavg = wrong_y_bits.iter().sum::<usize>() / wrong_y_bits.len();
    // let zavg = wrong_z_bits.iter().sum::<usize>() / wrong_z_bits.len();

    // dbg!(wrong_count);
    // // dbg!(wrong_x_bits);
    // // dbg!(wrong_y_bits);
    // // dbg!(wrong_z_bits);
    // dbg!(xavg, yavg, zavg);

    // let xoutliers = wrong_x_bits.iter().positions(|count| *count > 4 * xavg).collect_vec();
    // let youtliers = wrong_y_bits.iter().positions(|count| *count > 4 * yavg).collect_vec();
    // let zoutliers = wrong_z_bits.iter().positions(|count| *count > 4 * zavg).collect_vec();

    // dbg!(xoutliers, youtliers, zoutliers);

    dbg!(wires.get("z00"));

    let reverse = wires
        .iter()
        .flat_map(|(k, v)| {
            match v {
                Wire::Constant(a) => vec![],
                Wire::And(a, b) => vec![
                    (a.to_string(), k.to_string()),
                    (b.to_string(), k.to_string()),
                ],
                Wire::Or(a, b) => vec![
                    (a.to_string(), k.to_string()),
                    (b.to_string(), k.to_string()),
                ],
                Wire::Xor(a, b) => vec![
                    (a.to_string(), k.to_string()),
                    (b.to_string(), k.to_string()),
                ],
            }
        })
        .into_group_map();

    dbg!(resolve_outputs(&reverse, &["y44".to_string(), "x44".to_string()]));
    dbg!(resolve_inputs(wires, &["z00".to_string()]));
    // dbg!(resolve_inputs(wires, "z01"));
    // dbg!(resolve_inputs(wires, "z01").difference(&resolve_inputs(wires, "z00")).collect_vec());


    // let z30_inputs = resolve_inputs(wires, &["z30".to_string()]).into_iter().filter(|wire| wire.starts_with("x") || wire.starts_with("y")).collect_vec();
    // dbg!(z30_inputs);


    // for i in (0..45).rev() {
    //     let x = format!("x{i:02}");
    //     let y = format!("y{i:02}");

    //     let expected_z = (i..46).map(|j| format!("z{j:02}")).collect::<HashSet<_>>();
    //     let actual_z = resolve_outputs(&reverse, &[x.clone(), y.clone()]).into_iter().filter(|wire| wire.starts_with("z")).collect();

    //     if actual_z != expected_z {
    //         dbg!(i, &x, &y);
    //         dbg!(expected_z.difference(&actual_z).collect_vec());
    //         dbg!(actual_z.difference(&expected_z).collect_vec());
    //     }
    // }


    validate(wires.clone(), 46, 0);

    // let mut valid = HashSet::new();

    // for i in 0..45 {
    //     // dbg!(i);
    //     let test_cases = [
    //         (1 << i, 0),
    //         (0, 1 << i),
    //         (1 << i, 1 << i),
    //         // ((((1 << 45) - 1) ^ ((1 << i) - 1)), 1 << i),
    //         // (1 << i, (((1 << 45) - 1) ^ ((1 << i) - 1))),
    //         // ((((1 << 45) - 1) ^ ((1 << i) - 1)), (((1 << 45) - 1) ^ ((1 << i) - 1))),
    //         (((1 << (i + 1)) - 1), 0),
    //         (0, ((1 << (i + 1)) - 1)),
    //         (((1 << (i + 1)) - 1), ((1 << (i + 1)) - 1)),
    //     ];

    //     // let outputs = resolve_outputs(&reverse, &[format!("x{i:02}"), format!("y{i:02}")]);
    //     // let inputs = resolve_inputs(wires, &format!("z{i:02}"));

    //     if let Some(diff) = test_cases
    //         .into_iter()
    //         .filter_map(|(x, y)| {
    //             let mut wires = wires.clone();
    //             set_signal(&mut wires, "x", x);
    //             set_signal(&mut wires, "y", y);
    //             let z_expected = x + y;
    //             let z_actual = get_signal(&wires, "z").unwrap();
    //             if z_actual != z_expected {
    //                 dbg!(i, x, y, z_expected, z_actual);
    //                 eprintln!("-- ze: 0b{z_expected:045b}");
    //                 eprintln!("-- za: 0b{z_actual:045b}");
    //             }

    //             least_significant_diff(z_expected, z_actual)
    //         })
    //         .min()
    //     {
    //         let valid_outputs = (0..diff)
    //             .map(|i| format!("z{i:02}"))
    //             .collect_vec();
    //         let valid_inputs = resolve_inputs(wires, &valid_outputs);
    //         let inputs_at_diff = resolve_inputs(wires, &[format!("z{diff:02}")]);
    //         let candidates = inputs_at_diff.difference(&valid_inputs).collect_vec();

    //         dbg!(i, diff, candidates);

    //         break;
    //     }

    //     // if test_cases.into_iter().all(|(x, y)| {
    //     //     let mut wires = wires.clone();
    //     //     set_signal(&mut wires, "x", x);
    //     //     set_signal(&mut wires, "y", y);
    //     //     let z_expected = x + y;
    //     //     let z_actual = get_signal(&wires, "z").unwrap();
    //     //     if z_actual != z_expected {
    //     //         dbg!(i, x, y, z_expected, z_actual);
    //     //         eprintln!("-- ze: 0b{z_expected:045b}");
    //     //         eprintln!("-- za: 0b{z_actual:045b}");
    //     //     }

    //     //     z_actual == z_expected
    //     // }) {
    //     //     valid.extend(inputs);
    //     // } else {
    //     //     let candidates = inputs.difference(&valid).collect_vec();
    //     //     dbg!(&candidates);
    //     //     dbg!(candidates.iter().combinations(2).count());

    //     //     for (a, b) in candidates.iter().tuple_combinations() {
    //     //         eprintln!("---------------------------");
    //     //         dbg!(a, b);
    //     //         let mut wires = wires.clone();
    //     //         let awire = wires.get(*a).unwrap().clone();
    //     //         let bwire = wires.get(*b).unwrap().clone();
    //     //         wires.insert(a.to_string(), bwire);
    //     //         wires.insert(b.to_string(), awire);

    //     //         if has_cycle(&wires) {
    //     //             continue;
    //     //         }

    //     //         validate(wires);
    //     //     }
    //     //     break;
    //     // }
    // }

    unimplemented!()
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

    const EXAMPLE3: &str = indoc! {"
        x00: 0
        x01: 1
        x02: 0
        x03: 1
        x04: 0
        x05: 1
        y00: 0
        y01: 0
        y02: 1
        y03: 1
        y04: 0
        y05: 1

        x00 AND y00 -> z05
        x01 AND y01 -> z02
        x02 AND y02 -> z01
        x03 AND y03 -> z03
        x04 AND y04 -> z04
        x05 AND y05 -> z00
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

    // #[test]
    // fn part2_example3() {
    //     assert_eq!("z00,z01,z02,z05", part2(&parse(EXAMPLE3).unwrap()).unwrap());
    // }

    #[test]
    fn part2_input() {
        assert_eq!("btb,cmv,mwp,rdg,rmj,z17,z23,z30", part2(&parse(include_str!("../input/2024/day24.txt")).unwrap()).unwrap());
    }
}
