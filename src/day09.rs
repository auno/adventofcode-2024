use std::iter::repeat_n;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DiskObject {
    File(usize, usize),
    Free(usize),
}

impl DiskObject {
    fn len(&self) -> usize {
        match *self {
            DiskObject::File(len, _) => len,
            DiskObject::Free(len) => len,
        }
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<DiskObject>> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_digit(10)
                .map(|d| (i, d as usize))
                .context(format!("Unable to parse block size: {c}"))
        })
        .map_ok(|(i, len)| {
            if i % 2 == 0 {
                DiskObject::File(len, i / 2)
            } else {
                DiskObject::Free(len)
            }
        })
        .collect()
}

fn generate_disk_map(input: &[DiskObject]) -> Vec<Option<usize>> {
    input
        .iter()
        .flat_map(|disk_object| {
            match *disk_object {
                DiskObject::File(len, id) => repeat_n(Some(id), len),
                DiskObject::Free(len) => repeat_n(None, len),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[DiskObject]) -> usize {
    let mut disk_map = generate_disk_map(input);

    let mut a = 0;
    let mut b = disk_map.len() - 1;

    while b > a {
        match (disk_map[a], disk_map[b]) {
            (Some(_), _) => {
                a += 1;
            },
            (None, Some(_)) => {
                disk_map.swap(a, b);
                a += 1;
                b -= 1;
            },
            (None, None) => {
                b -= 1;
            },
        }
    }

    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|f| i * f))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[DiskObject]) -> usize {
    let mut disk = input.to_vec();
    let file_ids = disk
        .iter()
        .enumerate()
        .filter_map(|(i, disk_object)| {
            match *disk_object {
                DiskObject::File(_, id) => Some((i, id)),
                DiskObject::Free(_) => None,
            }
        })
        .collect_vec();

    for &(initial_file_pos, id) in file_ids.iter().rev() {
        let Some((file_pos, &file)) = disk[initial_file_pos..]
            .iter()
            .find_position(|disk_object| {
                match *disk_object {
                    DiskObject::File(_, candidate_id) => *candidate_id == id,
                    DiskObject::Free(_) => false,
                }
            })
            .map(|(file_pos, file)| (file_pos + initial_file_pos, file))
        else {
            panic!("Unable to find file: {id}");
        };

        let Some((free_pos, &free)) = disk.iter().take(file_pos).find_position(|disk_object| {
            match *disk_object {
                DiskObject::File(_, _) => false,
                DiskObject::Free(candidate_len) => *candidate_len >= file.len(),
            }
        }) else {
            continue;
        };

        disk[file_pos] = DiskObject::Free(file.len());
        disk[free_pos] = file;

        if free.len() > file.len() {
            disk.insert(free_pos + 1, DiskObject::Free(free.len() - file.len()));
        }
    }

    generate_disk_map(&disk)
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|f| i * f))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "2333133121414131402";

    #[test]
    fn generate_disk_map_example1() {
        let disk_map = generate_disk_map(&parse(EXAMPLE1).unwrap());
        let string_rep = disk_map
            .iter()
            .map(|b| match b {
                Some(f) => f.to_string(),
                None => ".".to_string(),
            })
            .collect::<String>();
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", string_rep);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(1928, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6378826667552, part1(&parse(include_str!("../input/2024/day9.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(2858, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    #[ignore]
    fn part2_input() {
        assert_eq!(6413328569890, part2(&parse(include_str!("../input/2024/day9.txt")).unwrap()));
    }
}
