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

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d18/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: Board = HashMap::new();
    let mut size = 0;

    let mut position = Coord::new(0, 0);
    let mut dir_last: Option<Dir> = None;
    let mut path: Vec<(Coord, Option<Dir>)> = vec![(position, None)];

    let mut turning = 0;

    let mut min = Coord::new(0, 0);
    let mut max = Coord::new(0, 0);

    for (row, l) in buf_reader.lines().enumerate() {
        let l = l?;
        let mut comps = l.split_whitespace();
        let dir = comps.next().unwrap();
        let steps = comps.next().unwrap();
        let _ = comps.next().unwrap();

        let dir = Dir::new(dir.chars().next().unwrap());
        let steps = u32::from_str_radix(steps, 10).unwrap();

        for _ in 0..steps {
            position = position.m(dir).unwrap();
            path.push((position, Some(dir)));
            b.insert(position, 0);

            match dir_last {
                Some(dir_last) => turning += dir_last.turning_number(&dir).unwrap(),
                None => {}
            }

            dir_last = Some(dir);

            min.row = min.row.min(position.row);
            min.col = min.col.min(position.col);

            max.col = max.col.max(position.col);
            max.row = max.row.max(position.row)
        }
    }

    let mut filled = b.clone();

    for (c, dir) in path {
        if let Some(d) = dir {
            let turn_dir = d.turn(turning);
            let mut inner = c;
            loop {
                inner = inner.m(turn_dir).unwrap();
                if b.contains_key(&inner) {
                    break;
                }
                filled.insert(inner, 0);
            }
        }

        // for row in min.row..=max.row {
        //     for col in min.col..=max.col {
        //         if b.contains_key(&Coord::new(row, col)) {
        //             print!("#")
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!("");
        // }

        // println!("-------");
    }

    for row in min.row..=max.row {
        for col in min.col..=max.col {
            if b.contains_key(&Coord::new(row, col)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }

    println!("{}", filled.len());

    Ok(())
}
