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

fn explore(b: &Board, start: &Coord) -> usize {
    let mut set: HashSet<Coord> = HashSet::new();

    set.insert(*start);
    for i in 0..131 {
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
        println!("ITER {}: {}", i, set.len())
    }

    set.len()
}

pub fn p2() -> std::io::Result<()> {
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

    let N = 2022999;

    let inner_odd = 7424;
    let mut inner_odd_num: usize = 1;
    for i in 0..N {
        if i % 2 == 0 {
            inner_odd_num += 4 * i;
        }
    }
    let inner_even = 7388;
    let mut inner_even_num: usize = 0;
    for i in 0..N {
        if i % 2 == 1 {
            inner_even_num += 4 * i;
        }
    }

    let point_left = explore(&b, &Coord::new(65, 131));
    let point_right = explore(&b, &Coord::new(65, 0));
    let point_up = explore(&b, &Coord::new(131, 65));
    let point_down = explore(&b, &Coord::new(0, 65));

    let perim_ul = explore(&b, &Coord::new(131, 131));
    let perim_ur = explore(&b, &Coord::new(131, 0));
    let perim_dl = explore(&b, &Coord::new(0, 131));
    let perim_dr = explore(&b, &Coord::new(0, 0));

    let points = point_left + point_right + point_up + point_down;
    let perim =
        ((N - 1) * perim_ul) + ((N - 1) * perim_ur) + ((N - 1) * perim_dl) + ((N - 1) * perim_dr);

    let inner = inner_odd_num * inner_odd + inner_even_num * inner_even;

    let sum = points + perim + inner;

    println!("{sum}");

    Ok(())
}
