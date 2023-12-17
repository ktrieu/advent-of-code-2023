use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord(i64, i64);

impl Coord {
    pub fn valid(&self, size: i64) -> bool {
        self.0 >= 0 && self.0 < size && self.1 >= 0 && self.1 < size
    }

    pub fn new(row: i64, col: i64) -> Self {
        Self(row, col)
    }

    pub fn increment(&self, other: &Coord, size: i64) -> Option<Self> {
        let ret = Coord(self.0 + other.0, self.1 + other.1);

        if ret.valid(size) {
            Some(ret)
        } else {
            None
        }
    }
}

type Board = HashMap<Coord, char>;

pub fn cast(
    b: &Board,
    location: Coord,
    dir: Coord,
    size: i64,
    ret: HashSet<(Coord, Coord)>,
) -> HashSet<(Coord, Coord)> {
    let mut pos = Some(location);
    let mut dir = dir;
    let mut ret = ret.clone();

    while let Some(p) = pos {
        if ret.contains(&(p, dir)) {
            println!("break");
            break;
        }
        println!("{} {}", p.0, p.1);
        ret.insert((p, dir));
        let c = b.get(&p).unwrap();
        match c {
            '.' => {
                pos = p.increment(&dir, size);
            }
            '/' => {
                dir = match dir {
                    Coord(0, 1) => Coord::new(-1, 0),
                    Coord(0, -1) => Coord::new(1, 0),
                    Coord(1, 0) => Coord::new(0, -1),
                    Coord(-1, 0) => Coord::new(0, 1),
                    _ => unreachable!(),
                };
                pos = p.increment(&dir, size);
            }
            '\\' => {
                dir = match dir {
                    Coord(0, 1) => Coord::new(1, 0),
                    Coord(0, -1) => Coord::new(-1, 0),
                    Coord(1, 0) => Coord::new(0, 1),
                    Coord(-1, 0) => Coord::new(0, -1),
                    _ => unreachable!(),
                };
                pos = p.increment(&dir, size);
            }
            '|' => {
                if dir.1 == 0 {
                    pos = p.increment(&dir, size);
                } else {
                    let first_dir = Coord::new(1, 0);
                    let second_dir = Coord::new(-1, 0);

                    if let Some(first_pos) = p.increment(&first_dir, size) {
                        ret.extend(cast(b, first_pos, first_dir, size, ret.clone()));
                    }

                    if let Some(second_pos) = p.increment(&second_dir, size) {
                        ret.extend(cast(b, second_pos, second_dir, size, ret.clone()));
                    }

                    break;
                }
            }
            '-' => {
                if dir.0 == 0 {
                    pos = p.increment(&dir, size);
                } else {
                    let first_dir = Coord::new(0, -1);
                    let second_dir = Coord::new(0, 1);

                    if let Some(first_pos) = p.increment(&first_dir, size) {
                        ret.extend(cast(b, first_pos, first_dir, size, ret.clone()));
                    }

                    if let Some(second_pos) = p.increment(&second_dir, size) {
                        ret.extend(cast(b, second_pos, second_dir, size, ret.clone()));
                    }

                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    ret
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d16/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: Board = HashMap::new();
    let mut size = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        for (col, c) in l?.chars().enumerate() {
            b.insert(Coord::new(row as i64, col as i64), c);
        }
        size += 1;
    }

    let ret: HashSet<(Coord, Coord)> = HashSet::new();

    let r = cast(&b, Coord::new(0, 0), Coord::new(0, 1), size, ret);

    // for row in 0..size {
    //     for col in 0..size {
    //         let c = Coord::new(row, col);
    //         if r.contains(&c) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    let h: HashSet<Coord> = r.iter().map(|t| t.0).collect();

    println!("{}", h.len());

    Ok(())
}
