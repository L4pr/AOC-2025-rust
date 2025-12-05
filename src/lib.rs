use std::cmp::{max, Ordering};
use std::fmt;
use std::ops::Index;
pub use itertools::Itertools;

pub mod template;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Dir {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Dir::Up        => ( 0, -1),
            Dir::UpRight   => ( 1, -1),
            Dir::Right     => ( 1,  0),
            Dir::DownRight => ( 1,  1),
            Dir::Down      => ( 0,  1),
            Dir::DownLeft  => (-1,  1),
            Dir::Left      => (-1,  0),
            Dir::UpLeft    => (-1, -1),
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Dir::Up        => Dir::Right,
            Dir::UpRight   => Dir::DownRight,
            Dir::Right     => Dir::Down,
            Dir::DownRight => Dir::DownLeft,
            Dir::Down      => Dir::Left,
            Dir::DownLeft  => Dir::UpLeft,
            Dir::Left      => Dir::Up,
            Dir::UpLeft    => Dir::UpRight,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Dir::Up        => Dir::Left,
            Dir::UpRight   => Dir::UpLeft,
            Dir::Right     => Dir::Up,
            Dir::DownRight => Dir::UpRight,
            Dir::Down      => Dir::Right,
            Dir::DownLeft  => Dir::DownRight,
            Dir::Left      => Dir::Down,
            Dir::UpLeft    => Dir::DownLeft,
        }
    }

    pub fn turn_45_right(&self) -> Self {
        match self {
            Dir::Up        => Dir::UpRight,
            Dir::UpRight   => Dir::Right,
            Dir::Right     => Dir::DownRight,
            Dir::DownRight => Dir::Down,
            Dir::Down      => Dir::DownLeft,
            Dir::DownLeft  => Dir::Left,
            Dir::Left      => Dir::UpLeft,
            Dir::UpLeft    => Dir::Up,
        }
    }

    pub fn turn_45_left(&self) -> Self {
        match self {
            Dir::Up        => Dir::UpLeft,
            Dir::UpRight   => Dir::Up,
            Dir::Right     => Dir::UpRight,
            Dir::DownRight => Dir::Right,
            Dir::Down      => Dir::DownRight,
            Dir::DownLeft  => Dir::Down,
            Dir::Left      => Dir::DownLeft,
            Dir::UpLeft    => Dir::Left,
        }
    }

    pub const ALL: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

    pub const ALL_WITH_DIAGONALS: [Dir; 8] = [
        Dir::Up, Dir::UpRight, Dir::Right, Dir::DownRight,
        Dir::Down, Dir::DownLeft, Dir::Left, Dir::UpLeft
    ];
}

pub fn move_in_direction(direction: Dir, current_location: &(isize, isize)) -> (isize, isize) {
    (
        current_location.0 + direction.delta().0,
        current_location.1 + direction.delta().1,
    )
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn fast_parse(bytes: &[u8]) -> i32 {
    let mut n = 0;
    for &b in bytes {
        n = n * 10 + (b - b'0') as i32;
    }
    n
}

#[inline(always)]
pub fn merge_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_ranges: Vec<(u64, u64)> = Vec::new();

    ranges.sort_by_key(|r| r.0);

    for range in ranges {
        if new_ranges.is_empty() {
            new_ranges.push((range.0, range.1));
            continue;
        }

        let last_index = new_ranges.len() - 1;
        let last_range = &mut new_ranges[last_index];

        if range.0 <= last_range.1 + 1 {
            last_range.1 = max(last_range.1, range.1);
        } else {
            new_ranges.push((range.0, range.1));
        }
    }

    new_ranges
}

pub fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect_vec()
}

pub fn make_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec()
}

pub fn make_grid_int(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| (b - b'0') as u32)
                .collect()
        })
        .collect_vec()
}

pub fn get_area_polygon(corners: Vec<(i32, i32)>) -> u64 {
    let sum: i64 = corners.iter()
        .circular_tuple_windows()
        .map(|(p1, p2)| (p1.0 as i64 * p2.1 as i64) - (p2.0 as i64 * p1.1 as i64))
        .sum();

    (sum.abs() / 2) as u64
}

// Priority queue
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len(), "Data size does not match dimensions");
        Self { width, height, data }
    }

    // F is a function that takes a byte (u8) and returns a T
    pub fn parse_with<F>(input: &str, transform: F) -> Self
    where
        F: Fn(u8) -> T,
    {
        let mut data = Vec::with_capacity(input.len());
        let mut height = 0;
        let mut width = 0;

        for line in input.lines() {
            if width == 0 { width = line.len(); }
            data.extend(line.bytes().map(&transform));
            height += 1;
        }

        Self { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.data[y * self.width + x])
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
    
    pub fn get_isize(&self, x: isize, y: isize) -> &T {
        &self.data[(y * self.width as isize + x) as usize]
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: T) -> bool {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            true
        } else {
            false
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn neighbors_locations(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        Dir::ALL.iter().filter_map(move |&dir| {
            self.step(x, y, dir)
        })
    }

    pub fn neighbors_with_diagonals_locations(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        Dir::ALL_WITH_DIAGONALS.iter().filter_map(move |&dir| {
            self.step(x, y, dir)
        })
    }

    pub fn neighbors_with_dir_locations(&self, x: usize, y: usize) -> impl Iterator<Item = (Dir, (usize, usize))> + '_ {
        Dir::ALL.iter().filter_map(move |&dir| {
            self.step(x, y, dir).map(|coord| (dir, coord))
        })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = &T> + '_ {
        self.neighbors_locations(x, y)
            .filter_map(move |(nx, ny)| {
                self.get(nx, ny)
            })
    }

    pub fn neighbors_with_diagonals(&self, x: usize, y: usize) -> impl Iterator<Item = &T> + '_ {
        self.neighbors_with_diagonals_locations(x, y)
            .filter_map(move |(nx, ny)| {
                self.get(nx, ny)
            })
    }

    #[inline(always)]
    pub fn step(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        let (dx, dy) = dir.delta();

        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;

        if nx >= 0 && nx < (self.width as isize)
            && ny >= 0 && ny < (self.height as isize) {
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, target: &T) -> Option<(usize, usize)> {
        self.data.iter().position(|r| r == target)
            .map(|idx| (idx % self.width, idx / self.width))
    }

    pub fn find_all(&self, target: &T) -> Vec<(usize, usize)> {
        self.data.iter().enumerate()
            .filter(|(_, r)| *r == target)
            .map(|(idx, _)| (idx % self.width, idx / self.width))
            .collect()
    }
}

// Use this file to add helper functions and additional modules.
