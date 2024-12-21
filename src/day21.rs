use std::iter::{once, repeat_n};

use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum NumPadKey {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<char> for NumPadKey {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' => Ok(NumPadKey::A),
            '0' => Ok(NumPadKey::Zero),
            '1' => Ok(NumPadKey::One),
            '2' => Ok(NumPadKey::Two),
            '3' => Ok(NumPadKey::Three),
            '4' => Ok(NumPadKey::Four),
            '5' => Ok(NumPadKey::Five),
            '6' => Ok(NumPadKey::Six),
            '7' => Ok(NumPadKey::Seven),
            '8' => Ok(NumPadKey::Eight),
            '9' => Ok(NumPadKey::Nine),
            _ => bail!("Unrecognized NumPadKey: {value}"),
        }
    }
}

impl TryFrom<(isize, isize)> for NumPadKey {
    type Error = Error;

    fn try_from(value: (isize, isize)) -> Result<Self> {
        match value {
            (0, 0) => Ok(NumPadKey::Seven),
            (0, 1) => Ok(NumPadKey::Eight),
            (0, 2) => Ok(NumPadKey::Nine),
            (1, 0) => Ok(NumPadKey::Four),
            (1, 1) => Ok(NumPadKey::Five),
            (1, 2) => Ok(NumPadKey::Six),
            (2, 0) => Ok(NumPadKey::One),
            (2, 1) => Ok(NumPadKey::Two),
            (2, 2) => Ok(NumPadKey::Three),
            (3, 1) => Ok(NumPadKey::Zero),
            (3, 2) => Ok(NumPadKey::A),
            _ => panic!("Unrecognized NumPadKey: {value:?}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum DPadKey {
    A,
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for DPadKey {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' => Ok(DPadKey::A),
            '^' => Ok(DPadKey::Up),
            '>' => Ok(DPadKey::Right),
            'v' => Ok(DPadKey::Down),
            '<' => Ok(DPadKey::Left),
            _ => bail!("Unrecognized NumPadKey: {value}"),
        }
    }
}

impl TryFrom<(isize, isize)> for DPadKey {
    type Error = Error;

    fn try_from(value: (isize, isize)) -> Result<Self> {
        match value {
            (0, 1) => Ok(DPadKey::Up),
            (0, 2) => Ok(DPadKey::A),
            (1, 0) => Ok(DPadKey::Left),
            (1, 1) => Ok(DPadKey::Down),
            (1, 2) => Ok(DPadKey::Right),
            _ => panic!("Unrecognized DPadKey: {value:?}"),
        }
    }
}

type Input = Vec<(String, Vec<NumPadKey>, usize)>;

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| Ok((
            line.to_string(),
            line.chars().map(NumPadKey::try_from).collect::<Result<_>>()?,
            line[0..line.len() - 1].parse::<usize>()?,
        )))
        .collect()
}

type NumPadLut = HashMap<(NumPadKey, NumPadKey), Vec<Vec<DPadKey>>>;
type DPadLut = HashMap<(DPadKey, DPadKey), Vec<Vec<DPadKey>>>;

fn generate_numpad_lut() -> NumPadLut {
    let mut nplut: NumPadLut = HashMap::new();

    for start in (0..4isize).cartesian_product(0..3isize) {
        if start == (3, 0) { continue; }

        for end in (0..4isize).cartesian_product(0..3isize) {
            if end == (3, 0) { continue; }

            let (si, sj) = start;
            let (ei, ej) = end;

            let mut paths = vec![];

            let idiff = si.abs_diff(ei);
            let jdiff = sj.abs_diff(ej);

            let ikey = if si > ei { DPadKey::Up } else { DPadKey::Down };
            let jkey = if sj > ej { DPadKey::Left } else { DPadKey::Right };

            if (ei, sj) != (3, 0) {
                paths.push(repeat_n(ikey, idiff).chain(repeat_n(jkey, jdiff)).chain(once(DPadKey::A)).collect_vec());
            }

            if (si, ej) != (3, 0) {
                paths.push(repeat_n(jkey, jdiff).chain(repeat_n(ikey, idiff)).chain(once(DPadKey::A)).collect_vec());
            }

            nplut.insert((start.try_into().unwrap(), end.try_into().unwrap()), paths.into_iter().unique().collect_vec());
        }
    }

    nplut
}

fn generate_dpad_lut() -> DPadLut {
    let mut dplut: DPadLut = HashMap::new();

    for start in (0..2isize).cartesian_product(0..3isize) {
        if start == (0, 0) { continue; }

        for end in (0..2isize).cartesian_product(0..3isize) {
            if end == (0, 0) { continue; }

            let (si, sj) = start;
            let (ei, ej) = end;

            let mut paths = vec![];

            let idiff = si.abs_diff(ei);
            let jdiff = sj.abs_diff(ej);

            let ikey = if si > ei { DPadKey::Up } else { DPadKey::Down };
            let jkey = if sj > ej { DPadKey::Left } else { DPadKey::Right };

            if (ei, sj) != (0, 0) {
                paths.push(repeat_n(ikey, idiff).chain(repeat_n(jkey, jdiff)).chain(once(DPadKey::A)).collect_vec());
            }

            if (si, ej) != (0, 0) {
                paths.push(repeat_n(jkey, jdiff).chain(repeat_n(ikey, idiff)).chain(once(DPadKey::A)).collect_vec());
            }

            dplut.insert((start.try_into().unwrap(), end.try_into().unwrap()), paths.into_iter().unique().collect_vec());
        }
    }

    dplut
}

fn find_best_dpad_sequence(cache: &mut HashMap<(Vec<DPadKey>, usize), usize>, dplut: &DPadLut, dpad_keys: &Vec<DPadKey>, n: usize) -> usize {
    if n == 0 { return dpad_keys.len(); }

    if let Some(cached_result) = cache.get(&(dpad_keys.clone(), n)) {
        return *cached_result;
    }

    let mut count = 0;
    let mut current = DPadKey::A;

    for &next in dpad_keys {
        let mut best = usize::MAX;

        for dpad_keys in dplut.get(&(current, next)).unwrap() {
            best = best.min(find_best_dpad_sequence(cache, dplut, dpad_keys, n - 1));
        }

        count += best;
        current = next;
    }

    cache.insert((dpad_keys.clone(), n), count);

    count
}

fn solve(input: &Input, num_robots: usize) -> usize {
    let nplut = generate_numpad_lut();
    let dplut = generate_dpad_lut();

    let mut sum = 0;
    let mut cache = HashMap::new();

    for (_, numpad_keys, numerical_part) in input {
        let mut count = 0;
        let mut current = NumPadKey::A;
        for &next in numpad_keys {
            let mut best = usize::MAX;

            for dpad_keys in nplut.get(&(current, next)).unwrap() {
                best = best.min(find_best_dpad_sequence(&mut cache, &dplut, dpad_keys, num_robots));
            }

            count += best;
            current = next;
        }

        sum += count * numerical_part;
    }

    sum
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> usize {
    solve(input, 2)
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> usize {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(126384, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(156714, part1(&parse(include_str!("../input/2024/day21.txt")).unwrap()));
    }


    #[test]
    fn part2_input() {
        assert_eq!(191139369248202, part2(&parse(include_str!("../input/2024/day21.txt")).unwrap()));
    }
}
