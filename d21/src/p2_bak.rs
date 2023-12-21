use std::{
    collections::{hash_map, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

    pub fn wrap(&self, size: &Coord) -> Option<Self> {
        let row = (self.row.abs() % size.row) - size.row;
        let col = (self.col.abs() % size.col) - size.col;
        if row != self.row || col != self.col {
            Some(Coord::new(row, col))
        } else {
            None
        }
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

struct Board {
    base: HashMap<Coord, char>,
    size: Coord,
}

impl Board {
    pub fn new(size: Coord) -> Self {
        Self {
            base: HashMap::new(),
            size,
        }
    }

    fn insert(&mut self, coord: &Coord, char: char) {
        self.base.insert(*coord, char);
    }

    fn get(&self, coord: &Coord) -> Option<&char> {
        println!("C {:?}", coord);
        let wrapped = coord.wrap(&self.size).unwrap_or(*coord);
        println!("W {:?}", wrapped);
        self.base.get(&wrapped)
    }
}

fn explore_board(b: &Board, start: &Coord, max: &mut Coord, min: &mut Coord) {
    let mut set: HashSet<Coord> = HashSet::new();

    set.insert(*start);

    for _ in 0..10 {
        let mut next_set: HashSet<Coord> = HashSet::new();
        for c in set.iter() {
            for d in DIRS.iter() {
                let next = c.m(*d);

                if let Some(n) = next {
                    if let Some(c) = b.get(&n) {
                        if *c != '#' {
                            next_set.insert(n);
                            max.row = max.row.max(n.row);
                            max.col = max.col.max(n.col);
                            min.row = min.row.min(n.row);
                            min.col = min.col.min(n.col);
                        }
                    }
                }
            }
        }
        set = next_set;
    }

    println!("{}", set.len());
}

// fn display(set: &HashSet<Coord>, b: &Board, size: &Coord) {
//     for row in 0..size.row {
//         for col in 0..size.col {
//             let c = if set.contains(&Coord::new(row, col)) {
//                 'X'
//             } else {
//                 *b.get(&Coord::new(row, col)).unwrap()
//             };

//             print!("{c}");
//         }
//         println!("");
//     }
// }

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d21/src/test.txt")?;
    let buf_reader = BufReader::new(file);

    let test_size = Coord::new(11, 11);
    let mut b: Board = Board::new(test_size);

    let mut start = Coord::new(0, 0);

    for (row, l) in buf_reader.lines().enumerate() {
        let row = row as i64;
        for (col, c) in l?.chars().enumerate() {
            let col = col as i64;
            if c == 'S' {
                start = Coord::new(row, col);
            }
            b.insert(&Coord::new(row, col), c);
        }
    }

    let mut max = Coord::new(i64::MIN, i64::MIN);
    let mut min = Coord::new(i64::MAX, i64::MAX);

    explore_board(&b, &start, &mut max, &mut min);

    println!("{:?}, {:?}", max, min);

    Ok(())
}
