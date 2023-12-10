use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Location {
    row: usize,
    col: usize,
}

impl Location {
    pub fn neighbor(&self, idx: usize, j: i64) -> Option<Self> {
        let row = self.row as i64;
        let col = self.col as i64;

        let (row, col) = match idx {
            0 => (row - j, col),
            1 => (row, col + j),
            2 => (row + j, col),
            3 => (row, col - j),
            _ => unreachable!(),
        };

        if row < 0 || col < 0 {
            None
        } else {
            Some(Self {
                row: row as usize,
                col: col as usize,
            })
        }
    }

    pub fn neighbors_all<'a>(&self, nodes: &'a HashMap<Location, Node>) -> Vec<Location> {
        let v = vec![
            self.neighbor(0, 1),
            self.neighbor(1, 1),
            self.neighbor(2, 1),
            self.neighbor(3, 1),
        ];

        v.iter().filter_map(|l| *l).collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    location: Location,
    // NORTH, EAST, SOUTH, WEST
    connections: [bool; 4],
}

impl Node {
    pub fn new(c: char, location: Location) -> Option<Self> {
        if c == '.' {
            return None;
        };

        if c == 'S' {
            return Some(Self {
                location,
                connections: [true, true, true, true],
            });
        };

        let connections = match c {
            '|' => [true, false, true, false],
            '-' => [false, true, false, true],
            'L' => [true, true, false, false],
            'J' => [true, false, false, true],
            '7' => [false, false, true, true],
            'F' => [false, true, true, false],
            _ => unreachable!(),
        };

        Some(Self {
            location,
            connections,
        })
    }

    pub fn neighbors<'a>(&self, nodes: &'a HashMap<Location, Node>) -> Vec<&'a Node> {
        let mut ret: Vec<&Node> = Vec::new();

        for i in 0..4 {
            if self.connections[i] {
                let l = self.location.neighbor(i, 2);
                match l {
                    Some(l) => {
                        if nodes.contains_key(&l) {
                            ret.push(&nodes[&l]);
                        }
                    }
                    None => {}
                }
            }
        }

        ret
    }
}

pub fn block(nodes: &HashMap<Location, Node>, l: (Option<Location>, Option<Location>)) -> bool {
    let n1 = l.0.and_then(|o| nodes.get(&o));
    let n2 = l.1.and_then(|o| nodes.get(&o));

    match (n1, n2) {
        (None, None) => return false,
        (None, Some(_)) => return false,
        (Some(_), None) => return false,
        (Some(n1), Some(n2)) => {
            let n1n = n1.neighbors(nodes);

            for n in n1n {
                if n.location == n2.location {
                    return true;
                }
            }
        }
    }

    false
}

pub fn ff<'a>(
    nodes: &'a HashMap<Location, Node>,
    pass: &HashMap<Location, bool>,
    start: &'a Node,
    rows: usize,
    cols: usize,
) ->  HashSet<Location>  {
    let mut stack: VecDeque<Location> = VecDeque::new();
    let mut dists: HashSet<Location> = HashSet::new();
    stack.push_back(start.location);
    dists.insert(start.location);

    let mut c = 0;

    while !stack.is_empty() {
        let n = stack.pop_front().unwrap();

        for ni in n.neighbors_all(nodes) {
            if dists.contains(&ni) {
                continue;
            }

            let p = match pass.get(&ni) {
                Some(p) => *p,
                None => true,
            };

            if !p {
                continue;
            }

            if ni.row >= rows || ni.col >= cols {
                continue
            }

            if ni.row % 2 == 0 && ni.col % 2 == 0 {
                c += 1;
            }

            dists.insert(ni);
            stack.push_back(ni);
        }
    }

    dists
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d10/src/test.txt")?;
    let buf_reader = BufReader::new(file);

    let mut nodes: HashMap<Location, Node> = HashMap::new();

    let mut start: Option<Node> = None;

    let mut cols = 0;
    let mut rows = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        cols = 0;
        rows += 1;
        for (col, c) in l?.chars().enumerate() {
            cols += 1;
            let loc = Location {
                row: (row * 2) + 1,
                col: (col * 2) + 1,
            };
            let node = Node::new(c, loc);
            match node {
                Some(node) => {
                    if c == 'S' {
                        start = Some(node);
                    }
                    nodes.insert(loc, node);
                }
                None => {}
            }
        }
    }

    let mut start = start.unwrap();

    for (idx, n) in start.neighbors(&nodes).iter().enumerate() {
        if !n
            .neighbors(&nodes)
            .iter()
            .any(|n| n.location == start.location)
        {
            start.connections[idx] = false;
        }
    }

    let mut pass: HashMap<Location, bool> = HashMap::new();

    for l in nodes.keys() {
        pass.insert(*l, false);
    }

    // for y in 0..(rows * 2) + 1 {
    //     for x in 0..(cols * 2) + 1 {
    //         match pass.get(&Location { row: y, col: x }) {
    //             Some(b) => match b {
    //                 true => print!("."),
    //                 false => print!("X"),
    //             },
    //             None => print!("."),
    //         }
    //     }
    //     println!("");
    // }

    for row in 0..(rows * 2) + 1 {
        for col in 0..(cols * 2) + 1 {
            if pass.contains_key(&Location { row, col }) {
                continue;
            }

            let fake = Location { row, col };

            let lhoriz = (fake.neighbor(3, 1), fake.neighbor(1, 1));

            let lvert = (fake.neighbor(0, 1), fake.neighbor(2, 1));

            let hblock = block(&nodes, lhoriz);
            let vblock = block(&nodes, lvert);

            let mut p = !(hblock || vblock);

            if lhoriz.0.is_some() && lhoriz.1.is_some() && lvert.0.is_some() && lvert.1.is_some() {
                // p = false;
            }

            pass.insert(fake, p);
        }
    }

    // for y in 0..(rows * 2) + 1 {
    //     for x in 0..(cols * 2) + 1 {
    //         match pass.get(&Location { row: y, col: x }) {
    //             Some(b) => match b {
    //                 true => print!("."),
    //                 false => print!("X"),
    //             },
    //             None => print!("."),
    //         }
    //     }
    //     println!("");
    // }

    let found = ff(
        &nodes,
        &pass,
        &Node {
            location: Location { row: 0, col: 0 },
            connections: [true, true, true, true],
        },
        (rows * 2) + 1,
        (cols * 2) + 1,
    );

    for y in 0..(rows * 2) + 1 {
        for x in 0..(cols * 2) + 1 {
            match found.get(&Location { row: y, col: x }) {
                Some(b) => print!("X"),
                None => {
                    match nodes.get(&Location { row: y + 1 & !1, col: x + 1 & !1}) {
                        Some(_) => print!("X"),
                        None => print!("."),
                    }
                }
            }
        }
        println!("");
    }

    let mut c = 0;

    for y in 0..(rows * 2) + 1 {
        for x in 0..(cols * 2) + 1 {
            match found.get(&Location { row: y, col: x }) {
                Some(b) => {},
                None => {
                    match nodes.get(&Location { row: y & 1, col: x & 1}) {
                        Some(_) => {},
                        None => { c += 1},
                    }
                }
            }
        }
    }

    println!("{}", c / 2);

    Ok(())
}
