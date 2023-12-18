use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    row: i64,
    col: i64,
}

impl Coord {
    pub fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }

    pub fn add(&self, other: &Self) -> Self {
        return Coord::new(self.row + other.row, self.col + other.col);
    }

    pub fn m(&self, dir: Dir) -> Option<Self> {
        let n = self.add(&dir.delta());
        Some(n)
    }
}

const DIRS: [Dir; 4] = [Dir::Up, Dir::Left, Dir::Down, Dir::Right];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    pub fn delta(&self) -> Coord {
        match self {
            Dir::Up => Coord::new(-1, 0),
            Dir::Left => Coord::new(0, -1),
            Dir::Down => Coord::new(1, 0),
            Dir::Right => Coord::new(0, 1),
        }
    }

    pub fn new(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'R' => Self::Right,
            'L' => Self::Left,
            'D' => Self::Down,
            _ => unreachable!(),
        }
    }

    pub fn idx(&self) -> i64 {
        match self {
            Dir::Up => 0,
            Dir::Right => 1,
            Dir::Down => 2,
            Dir::Left => 3,
        }
    }

    pub fn from_hex_digit(c: char) -> Self {
        match c {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => unreachable!(),
        }
    }

    pub fn turning_number(&self, other: &Self) -> Option<i64> {
        match (self, other) {
            (Dir::Up, Dir::Up) => Some(0),
            (Dir::Up, Dir::Left) => Some(-1),
            (Dir::Up, Dir::Down) => None,
            (Dir::Up, Dir::Right) => Some(1),
            (Dir::Left, Dir::Up) => Some(1),
            (Dir::Left, Dir::Left) => Some(0),
            (Dir::Left, Dir::Down) => Some(-1),
            (Dir::Left, Dir::Right) => None,
            (Dir::Down, Dir::Up) => None,
            (Dir::Down, Dir::Left) => Some(1),
            (Dir::Down, Dir::Down) => Some(0),
            (Dir::Down, Dir::Right) => Some(-1),
            (Dir::Right, Dir::Up) => Some(-1),
            (Dir::Right, Dir::Left) => None,
            (Dir::Right, Dir::Down) => Some(1),
            (Dir::Right, Dir::Right) => Some(0),
        }
    }

    pub fn turn(&self, number: i64) -> Dir {
        if number > 0 {
            match self {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Up,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Down,
            }
        } else if number < 0 {
            match self {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
            }
        } else {
            unreachable!()
        }
    }
}

type Board = HashMap<Coord, u32>;

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d18/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: Board = HashMap::new();

    let mut position = Coord::new(0, 0);
    let mut dir_last: Option<Dir> = None;
    let mut path: Vec<(Coord, Option<Dir>)> = vec![(position, None)];

    let mut turning = 0;

    let mut min = Coord::new(0, 0);
    let mut max = Coord::new(0, 0);

    let mut len = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        let l = l?;
        let mut comps = l.split_whitespace();
        let dir = comps.next().unwrap();
        let steps = comps.next().unwrap();
        let hex = comps.next().unwrap();

        // let dir = Dir::new(dir.chars().next().unwrap());
        // let steps = i64::from_str_radix(steps, 10).unwrap();
        // println!("{}", hex);
        let hex = hex.replace(")", "");
        let hex = hex.replace("(", "");

        let dir = Dir::from_hex_digit(hex.chars().nth(6).unwrap());

        let steps = i64::from_str_radix(
            &hex.chars().collect::<Vec<char>>()[1..6]
                .iter()
                .collect::<String>(),
            16,
        )
        .unwrap();

        println!("{:?} {}", dir, steps);

        let delta = dir.delta();
        position.row += delta.row * steps;
        position.col += delta.col * steps;
        len += steps;

        dir_last = Some(dir);
        path.push((position, Some(dir)));
    }

    // We're adding the first point twice oh well
    path.pop();

    for p in &path {
        println!("{:?}", p);
    }

    let mut sum = 0;

    for i in 0..path.len() - 1 {
        let (a, _) = path[i];
        let (b, _) = path[i + 1];

        let subsum = (a.row + b.row) * (a.col - b.col);
        println!("{:?} -> {:?}: {}", a, b, subsum);
        sum += subsum;
    }

    // One last iteration for the last/first point
    let (a, _) = path.last().unwrap();
    let (b, _) = path.first().unwrap();

    let subsum = (a.row + b.row) * (a.col - b.col);
    println!("{:?} -> {:?}: {}", a, b, subsum);
    sum += subsum;

    let area = sum / 2;
    let i = area - (len / 2) + 1;

    let sol = i + len;

    println!("{}", sol);

    Ok(())
}
