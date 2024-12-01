use std::collections::HashMap;
use anyhow::{Context, Result};
use lazy_static::lazy_static;

const CHAR_WIDTH: usize = 5;
const CHAR_HEIGHT: usize = 6;
const KNOWN_CHARS: &str = "ABCEFGHIJKLOPRSUZ";

lazy_static! {
    static ref KNOWN_CHARS_RENDERED: String = [
        ".##..###...##..####.####..##..#..#..###...##.#..#.#.....##..###..###...###.#..#.####.",
        "#..#.#..#.#..#.#....#....#..#.#..#...#.....#.#.#..#....#..#.#..#.#..#.#....#..#....#.",
        "#..#.###..#....###..###..#....####...#.....#.##...#....#..#.#..#.#..#.#....#..#...#..",
        "####.#..#.#....#....#....#.##.#..#...#.....#.#.#..#....#..#.###..###...##..#..#..#...",
        "#..#.#..#.#..#.#....#....#..#.#..#...#..#..#.#.#..#....#..#.#....#.#.....#.#..#.#....",
        "#..#.###...##..####.#.....###.#..#..###..##..#..#.####..##..#....#..#.###...##..####.",
    ].join("");

    static ref KNOWN_CHARS_MAP: HashMap<u32, char> = hash_characters('#', &KNOWN_CHARS_RENDERED)
        .into_iter()
        .zip(KNOWN_CHARS.chars())
        .collect::<HashMap<_, _>>();
}

fn hash_characters(lit_pixel: char, input: &str) -> Vec<u32> {
    let num_chars = input.len() / CHAR_HEIGHT / CHAR_WIDTH;
    let line_len = CHAR_WIDTH * num_chars;
    let mut hashes = vec![0u32; num_chars];

    for (i, c) in input.char_indices() {
        if c == lit_pixel {
            let line_index = i / line_len;
            let line_offset = i % line_len;
            let char_index = line_offset / CHAR_WIDTH;
            let bit_index = line_index * CHAR_WIDTH + (line_offset % CHAR_WIDTH);
            hashes[char_index] |= 1 << bit_index;
        }
    }

    hashes
}

pub fn ocr(lit_pixel: char, input: &str) -> Result<String> {
    let resolved_characters = hash_characters(lit_pixel, input)
        .iter()
        .map(|hash| KNOWN_CHARS_MAP.get(hash).copied().context("Unknown character"))
        .collect::<Result<Vec<char>>>()?;

    Ok(resolved_characters.iter().collect())
}
