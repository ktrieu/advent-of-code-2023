use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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

    pub fn valid(&self, width: i64, height: i64) -> bool {
        return self.row >= 0 && self.row < height && self.col >= 0 && self.col < width;
    }

    pub fn add(&self, other: &Self) -> Self {
        return Coord::new(self.row + other.row, self.col + other.col);
    }

    pub fn m(&self, dir: Dir, width: i64, height: i64) -> Option<Self> {
        let n = self.add(&dir.delta());
        if n.valid(width, height) {
            Some(n)
        } else {
            None
        }
    }
}

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

    pub fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Left => Dir::Right,
            Dir::Down => Dir::Up,
            Dir::Right => Dir::Left,
        }
    }
}

const DIRS: [Dir; 4] = [Dir::Up, Dir::Left, Dir::Down, Dir::Right];

pub fn valid_dirs(hist: &Hist) -> Vec<Dir> {
    // dbg!(hist);
    let steps = hist.0;
    let ret = match hist.1 {
        Some(dir) => {
            if steps < 4 {
                vec![dir]
            } else if steps < 10 {
                DIRS.clone()
                    .into_iter()
                    .filter(|d| *d != dir.opposite())
                    .collect()
            } else {
                DIRS.clone()
                    .into_iter()
                    .filter(|d| *d != dir.opposite() && *d != dir)
                    .collect()
            }
        }
        None => DIRS.to_vec(),
    };

    // dbg!(&ret);
    ret
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Hist(u64, Option<Dir>);

impl Hist {
    pub fn add(&self, dir: Dir) -> Self {
        match self.1 {
            Some(sdir) => {
                if dir == sdir {
                    Self(self.0 + 1, Some(dir))
                } else {
                    Self(1, Some(dir))
                }
            }
            None => Self(1, Some(dir)),
        }
    }
}

#[derive(PartialEq, Eq)]
struct State(Coord, Hist, u32);

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

pub fn solve(b: &Board, start: Coord, target: Coord, width: i64, height: i64) -> u32 {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let initial_hist: Hist = Hist(0, None);

    let mut dists: HashMap<(Coord, Hist), u32> = HashMap::new();
    dists.insert((start, initial_hist), 0);

    let mut parent: HashMap<Coord, Coord> = HashMap::new();

    heap.push(State(start, initial_hist, 0));

    while !heap.is_empty() {
        let State(node, hist, cost) = heap.pop().unwrap();
        // dbg!(node);
        // dbg!(hist);
        // dbg!(cost);

        if node == target && hist.0 >= 4 {
            return cost;
        }

        let stored_cost = dists.get(&(node, hist)).unwrap_or(&u32::MAX);
        if cost > *stored_cost {
            continue;
        }

        for d in valid_dirs(&hist) {
            let next_coord = node.m(d, width, height);
            if let Some(next_coord) = next_coord {
                // dbg!(next_coord);
                let next_cost = cost + b.get(&next_coord).unwrap();
                let next_hist = hist.add(d);

                let stored_cost = dists.get(&(next_coord, next_hist)).unwrap_or(&u32::MAX);

                if next_cost < *stored_cost {
                    parent.insert(node, next_coord);
                    heap.push(State(next_coord, next_hist, next_cost));
                    dists.insert((next_coord, next_hist), next_cost);
                }
            }
        }
    }

    0
}

type Board = HashMap<Coord, u32>;

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d17/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: Board = HashMap::new();
    let mut height = 0;
    let mut width = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        let row = row as i64;
        width = 0;
        for (col, c) in l?.chars().enumerate() {
            let col = col as i64;
            b.insert(Coord::new(row, col), c.to_digit(10).unwrap());
            width += 1;
        }
        height += 1;
    }

    println!("{width}, {height}");

    let result = solve(
        &b,
        Coord::new(0, 0),
        Coord::new(height - 1, width - 1),
        width,
        height,
    );

    println!("{result}");

    Ok(())
}
