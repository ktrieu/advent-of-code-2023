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

    pub fn valid(&self, size: i64) -> bool {
        return self.row >= 0 && self.row < size && self.col >= 0 && self.col < size;
    }

    pub fn add(&self, other: &Self) -> Self {
        return Coord::new(self.row + other.row, self.col + other.col);
    }

    pub fn m(&self, dir: Dir, size: i64) -> Option<Self> {
        let n = self.add(&dir.delta());
        if n.valid(size) {
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
}

const DIRS: [Dir; 4] = [Dir::Up, Dir::Left, Dir::Down, Dir::Right];

pub fn valid_dirs(hist: &[Option<Dir>; 3]) -> Vec<Dir> {
    let must_turn = match hist {
        [Some(a), Some(b), Some(c)] => a == b && b == c,
        _ => false,
    };

    let last = hist.last().unwrap();
    let r = match last {
        Some(last) => {
            let opposite = match last {
                Dir::Up => Dir::Down,
                Dir::Left => Dir::Right,
                Dir::Down => Dir::Up,
                Dir::Right => Dir::Left,
            };

            DIRS.clone()
                .into_iter()
                .filter(|d| {
                    if must_turn && d == last {
                        return false;
                    };

                    *d != opposite
                })
                .collect()
        }
        None => DIRS.to_vec(),
    };

    r
}

#[derive(PartialEq, Eq)]
struct State(Coord, [Option<Dir>; 3], u32);

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

pub fn compute(b: &Board) -> HashMap<Coord, Vec<Coord>> {
    let start = Coord::new(0, 0);
    let mut visited: HashSet<(Coord, [Option<Dir>; 3])> = HashSet::new();
    let mut graph = HashMap::new();
    let mut queue: VecDeque<(Coord, [Option<Dir>; 3])> = VecDeque::new();
    queue.push_front((start, [None, None, None]));

    graph
}

pub fn solve(b: &Board, start: Coord, target: Coord, size: i64) -> u32 {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let initial_hist: [Option<Dir>; 3] = [None, None, None];

    let mut dists: HashMap<(Coord, [Option<Dir>; 3]), u32> = HashMap::new();
    dists.insert((start, [None, None, None]), 0);

    let mut parent: HashMap<Coord, Coord> = HashMap::new();

    heap.push(State(start, initial_hist, 0));

    let mut iters = 0;

    while !heap.is_empty() {
        let State(node, hist, cost) = heap.pop().unwrap();
        // dbg!(node);
        // dbg!(hist);
        // dbg!(cost);

        let dirs = valid_dirs(&hist);

        // dbg!(dirs);
        // println!("------");

        if node == target {
            // let mut node = target;
            // while node != start {
            //     println!("{node:?}");
            //     node = *parent.get(&node).unwrap();
            // }

            return cost;
        }

        let stored_cost = dists.get(&(node, hist)).unwrap_or(&u32::MAX);
        if cost > *stored_cost {
            continue;
        }

        for d in valid_dirs(&hist) {
            let next_coord = node.m(d, size);
            if let Some(next_coord) = next_coord {
                let next_cost = cost + b.get(&next_coord).unwrap();
                let next_hist = [hist[1], hist[2], Some(d)];

                let stored_cost = dists.get(&(next_coord, next_hist)).unwrap_or(&u32::MAX);
                if next_cost < *stored_cost {
                    parent.insert(next_coord, node);
                    heap.push(State(next_coord, next_hist, next_cost));
                    dists.insert((next_coord, next_hist), next_cost);
                }
            }
        }
        iters += 1;
    }

    0
}

type Board = HashMap<Coord, u32>;

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d17/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: Board = HashMap::new();
    let mut size = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        let row = row as i64;
        for (col, c) in l?.chars().enumerate() {
            let col = col as i64;
            b.insert(Coord::new(row, col), c.to_digit(10).unwrap());
        }
        size += 1;
    }

    let result = solve(&b, Coord::new(0, 0), Coord::new(size - 1, size - 1), size);

    println!("{result}");

    Ok(())
}
