#![allow(dead_code)]
#![allow(private_bounds)]

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
use strum::EnumIter;
pub use strum::IntoEnumIterator;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => bail!("Unable to parse Direction: {value}"),
        }
    }
}

impl Direction {
    pub fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Position(pub isize, pub isize);

impl Position {
    pub fn new(i: usize, j: usize) -> Position {
        Position(i as isize, j as isize)
    }

    pub fn step(self, dir: Direction) -> Position {
        let Position(i, j) = self;
        match dir {
            Direction::Up => Position(i - 1, j),
            Direction::Down => Position(i + 1, j),
            Direction::Left => Position(i, j - 1),
            Direction::Right => Position(i, j + 1),
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> where T: Clone {
    store: Vec<T>,
    rows: GridSize,
    cols: GridSize,
    len: GridSize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GridSize(usize);

impl From<isize> for GridSize {
    fn from(value: isize) -> Self {
        GridSize(value as usize)
    }
}

impl From<usize> for GridSize {
    fn from(value: usize) -> Self {
        GridSize(value)
    }
}

impl From<GridSize> for usize {
    fn from(value: GridSize) -> Self {
        value.0
    }
}

impl From<GridSize> for isize {
    fn from(value: GridSize) -> Self {
        value.0 as isize
    }
}

impl<T> Grid<T> where T: Default + Copy + Clone {
    pub fn new<S>(rows: S, cols: S) -> Grid<T> where S: Into<GridSize> + PartialOrd<isize> + Display {
        Self::new_with_value(rows, cols, T::default())
    }
}

impl<T> Grid<T> where T: Clone {
    pub fn new_with_value<S>(rows: S, cols: S, value: T) -> Grid<T> where S: Into<GridSize> + PartialOrd<isize> + Display {
        if rows < 0 || cols < 0 {
            panic!("Dimensions not non-negative: ({rows}, {cols})");
        }

        let rows: usize = rows.into().into();
        let cols: usize = cols.into().into();
        let len = rows * cols;

        Grid {
            store: vec![value; len],
            rows: rows.into(),
            cols: cols.into(),
            len: len.into(),
        }
    }

    fn from<S, I>(rows: S, cols: S, values: I) -> Grid<T> where
        S: Into<GridSize>,
        I: IntoIterator<Item = T>,
    {
        let rows: usize = rows.into().into();
        let cols: usize = cols.into().into();
        let len = rows * cols;
        let store = Vec::from_iter(values);

        if store.len() != len {
            panic!("Unable to construct Grid from {}, not correct amount of elements", std::any::type_name::<I>())
        }

        Grid {
            store,
            rows: rows.into(),
            cols: cols.into(),
            len: len.into(),
        }
    }

    pub fn dimensions<S>(&self) -> (S, S) where S: From<GridSize>, {
            (self.rows(), self.cols())
        }

    pub fn rows<S>(&self) -> S where S: From<GridSize> {
        S::from(self.rows)
    }

    pub fn cols<S>(&self) -> S where S: From<GridSize> {
        S::from(self.cols)
    }

    pub fn len<S>(&self) -> S where S: From<GridSize> {
        S::from(self.len)
    }

    pub fn get(&self, &Position(i, j): &Position) -> Option<&T> {
        if i < 0 || i >= self.rows.into() || j < 0 || j >= self.cols.into() {
            return None;
        }

        Some(&self[(i, j)])
    }

    pub fn get_mut(&mut self, &Position(i, j): &Position) -> Option<&mut T> {
        if i < 0 || i >= self.rows.into() || j < 0 || j >= self.cols.into() {
            return None;
        }

        Some(&mut self[(i, j)])
    }

    pub fn set(&mut self, &Position(i, j): &Position, value: T) {
        self[(i, j)] = value;
    }
}

impl<T, S> Index<(S, S)> for Grid<T> where T: Clone, S: Into<GridSize> {
    type Output = T;

    fn index(&self, (i, j): (S, S)) -> &Self::Output {
        let i: usize = i.into().into();
        let j: usize = j.into().into();
        let pos = i * self.cols::<usize>() + j;
        &self.store[pos]
    }
}

impl<T, S> IndexMut<(S, S)> for Grid<T> where T: Clone, S: Into<GridSize> {
    fn index_mut(&mut self, (i, j): (S, S)) -> &mut Self::Output {
        let i: usize = i.into().into();
        let j: usize = j.into().into();
        let pos = i * self.cols::<usize>() + j;
        &mut self.store[pos]
    }
}

pub struct GridIntoIter<T> where T: Copy {
    grid: Grid<T>,
    index: usize,
}

impl<T> Iterator for GridIntoIter<T> where T: Copy {
    type Item = (Position, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.len() {
            return None;
        }

        let index = self.index as isize;
        self.index += 1;
        let i = index / self.grid.cols::<isize>();
        let j = index % self.grid.cols::<isize>();
        Some((Position(i, j), self.grid[(i, j)]))
    }
}

impl<T> IntoIterator for Grid<T> where T: Copy{
    type Item = (Position, T);
    type IntoIter = GridIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIter {
            grid: self,
            index: 0,
        }
    }
}

pub struct GridRefIntoIter<'a, T> where T: Clone {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridRefIntoIter<'a, T> where T: Clone {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.len() {
            return None;
        }

        let index = self.index as isize;
        self.index += 1;
        let i = index / self.grid.cols::<isize>();
        let j = index % self.grid.cols::<isize>();
        Some((Position(i, j), &self.grid[(i, j)]))
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> where T: Clone {
    type Item = (Position, &'a T);
    type IntoIter = GridRefIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridRefIntoIter {
            grid: self,
            index: 0,
        }
    }
}

impl<T> Grid<T> where
    T: Copy + Clone + TryFrom<char>,
    Result<T, <T as TryFrom<char>>::Error>: Context<T, <T as TryFrom<char>>::Error>,
{
    pub fn parse_with_position_detection(input: &str, markers: &[char], replacement: T) -> Result<(Grid<T>, HashMap<char, Vec<Position>>)> {
        use anyhow::Ok;

        let rows = input.lines().count();
        let cols = input.lines().next().context("No input lines found")?.len();

        let (ts, ps) = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| line
                .chars()
                .enumerate()
                .map(move |(j, c)| {
                    if markers.contains(&c) {
                        Ok((replacement, Some((c, Position(i as isize, j as isize)))))
                    } else {
                        Ok((T::try_from(c).context(format!("Unable to parse character: {c}"))?, None))
                    }
                })
            )
            .process_results(|iter| iter.unzip::<_, _, Vec<_>, Vec<_>>())?;

        let positions = ps.into_iter().flatten().into_group_map();

        Ok((Grid::from(rows, cols, ts), positions))
    }
}
