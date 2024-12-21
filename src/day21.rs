use std::iter::{once, repeat_n};

use anyhow::{bail, Context, Error, Result};
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

impl DPadKey {
    fn opposite(&self) -> Self {
        match self {
            DPadKey::A => unreachable!("DPadKey::opposite() should never be called for DPadKey::A"),
            DPadKey::Up => DPadKey::Down,
            DPadKey::Right => DPadKey::Left,
            DPadKey::Down => DPadKey::Up,
            DPadKey::Left => DPadKey::Right,
        }
    }
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

fn dpkeys_to_string(keys: &[DPadKey]) -> String {
    keys.iter()
        .map(|key| match key {
            DPadKey::A => 'A',
            DPadKey::Up => '^',
            DPadKey::Right => '>',
            DPadKey::Down => 'v',
            DPadKey::Left => '<',
        })
        .collect()
}

fn npkeys_to_string(keys: &[NumPadKey]) -> String {
    keys.iter()
        .map(|key| match key {
            NumPadKey::A => 'A',
            NumPadKey::Zero => '0',
            NumPadKey::One => '1',
            NumPadKey::Two => '2',
            NumPadKey::Three => '3',
            NumPadKey::Four => '4',
            NumPadKey::Five => '5',
            NumPadKey::Six => '6',
            NumPadKey::Seven => '7',
            NumPadKey::Eight => '8',
            NumPadKey::Nine => '9',
        })
        .collect()
}

fn simulate(key_presses: &Vec<DPadKey>) {
    let mut key_presses = key_presses.clone();

    eprintln!("-> simulated: {}", dpkeys_to_string(&key_presses));

    for level in 0..2 {
        let mut pos: (isize, isize) = (0, 2);
        let mut next_foo = vec![];

        for key_press in &key_presses {
            match key_press {
                DPadKey::Up => pos = (pos.0 - 1, pos.1),
                DPadKey::Right => pos = (pos.0, pos.1 + 1),
                DPadKey::Down => pos = (pos.0 + 1, pos.1),
                DPadKey::Left => pos = (pos.0, pos.1 - 1),
                DPadKey::A => next_foo.push(match pos {
                    (0, 1) => DPadKey::Up,
                    (0, 2) => DPadKey::A,
                    (1, 0) => DPadKey::Left,
                    (1, 1) => DPadKey::Down,
                    (1, 2) => DPadKey::Right,
                    _ => panic!("Illegal move at level {level}"),
                }),
            }
        }

        key_presses = next_foo;
        eprintln!("-- simulated: {}", dpkeys_to_string(&key_presses));
    }

    let mut bar = vec![];
    let mut pos: (isize, isize) = (3, 2);

    for key_press in &key_presses {
        eprintln!("   {pos:?} {key_press:?}");
        match key_press {
            DPadKey::Up => pos = (pos.0 - 1, pos.1),
            DPadKey::Right => pos = (pos.0, pos.1 + 1),
            DPadKey::Down => pos = (pos.0 + 1, pos.1),
            DPadKey::Left => pos = (pos.0, pos.1 - 1),
            DPadKey::A => bar.push(match pos {
                (0, 0) => NumPadKey::Seven,
                (0, 1) => NumPadKey::Eight,
                (0, 2) => NumPadKey::Nine,
                (1, 0) => NumPadKey::Four,
                (1, 1) => NumPadKey::Five,
                (1, 2) => NumPadKey::Six,
                (2, 0) => NumPadKey::One,
                (2, 1) => NumPadKey::Two,
                (2, 2) => NumPadKey::Three,
                (3, 1) => NumPadKey::Zero,
                (3, 2) => NumPadKey::A,
                _ => panic!("Illegal move: {pos:?}"),
            }),
        }
    }

    eprintln!("-= simulated: {}", npkeys_to_string(&bar));
}

type NumPadLut = HashMap<(NumPadKey, NumPadKey), Vec<Vec<DPadKey>>>;
type DPadLut = HashMap<(DPadKey, DPadKey), Vec<Vec<DPadKey>>>;

#[aoc(day21, part1)]
fn part1(input: &Input) -> Result<usize> {
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

            nplut.insert((start.try_into()?, end.try_into()?), paths.into_iter().unique().collect_vec());
        }
    }

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

            dplut.insert((start.try_into()?, end.try_into()?), paths.into_iter().unique().collect_vec());
        }
    }

    let mut sum = 0;

    for (code, numpad_keys, numerical_part) in input {
        eprintln!("-- code: {code} - {numpad_keys:?}");

        let mut sequences = find_all_numpad_sequences(&nplut, numpad_keys, NumPadKey::A);
        for sequence in &sequences {
            eprintln!("     {}", dpkeys_to_string(sequence));
        }

        for _round in 0..2 {
            sequences = sequences
                .into_iter()
                .flat_map(|sequence| find_all_dpad_sequences(&dplut, &sequence, DPadKey::A))
                .collect_vec();
            eprintln!("  {_round}: {}", dpkeys_to_string(&sequences[0]));
        }

        let shortest_length = sequences
            .iter()
            .map(|sequence| sequence.len())
            .min()
            .context(format!("No sequence found for {code}"))?;

        eprintln!("-- {code} => {shortest_length} * {numerical_part} = {}", shortest_length * numerical_part);

        sum += shortest_length * numerical_part
    }

    Ok(sum)
}

fn find_all_numpad_sequences(nplut: &NumPadLut, numpad_keys: &[NumPadKey], current: NumPadKey) -> Vec<Vec<DPadKey>> {
    let mut sequences = vec![];

    if numpad_keys.is_empty() {
        return vec![vec![]];
    }

    let next = numpad_keys[0];

    for prefix in nplut.get(&(current, next)).unwrap_or_else(|| panic!("No nplut entry for {current:?} -> {next:?}")) {
        for suffix in find_all_numpad_sequences(nplut, &numpad_keys[1..], next) {
            sequences.push(prefix.iter().chain(suffix.iter()).copied().collect_vec());
        }
    }

    sequences
}

fn find_all_dpad_sequences(dplut: &DPadLut, dpad_keys: &[DPadKey], current: DPadKey) -> Vec<Vec<DPadKey>> {
    let mut sequences = vec![];

    if dpad_keys.is_empty() {
        return vec![vec![]];
    }

    let next = dpad_keys[0];

    for prefix in dplut.get(&(current, next)).unwrap_or_else(|| panic!("No nplut entry for {current:?} -> {next:?}")) {
        for suffix in find_all_dpad_sequences(dplut, &dpad_keys[1..], next) {
            sequences.push(prefix.iter().chain(suffix.iter()).copied().collect_vec());
        }
    }

    sequences
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
        assert_eq!(126384, part1(&parse(EXAMPLE1).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(156714, part1(&parse(include_str!("../input/2024/day21.txt")).unwrap()).unwrap());
    }
}
