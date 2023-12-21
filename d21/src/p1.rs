use std::{
    collections::{hash_map, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
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
}

type Board = HashMap<Coord, char>;

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d21/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: Board = HashMap::new();

    let mut start = Coord::new(0, 0);

    for (row, l) in buf_reader.lines().enumerate() {
        let row = row as i64;
        for (col, c) in l?.chars().enumerate() {
            let col = col as i64;
            b.insert(Coord::new(row, col), c);
            if c == 'S' {
                start = Coord::new(row, col);
            }
        }
    }

    let mut set: HashSet<Coord> = HashSet::new();

    set.insert(start);

    for _ in 0..64 {
        let mut next_set: HashSet<Coord> = HashSet::new();
        for c in set.iter() {
            for d in DIRS.iter() {
                let next = c.m(*d);

                if let Some(n) = next {
                    if let Some(c) = b.get(&n) {
                        if *c != '#' {
                            next_set.insert(n);
                        }
                    }
                }
            }
        }
        set = next_set;
    }

    println!("{}", set.len());

    Ok(())
}
