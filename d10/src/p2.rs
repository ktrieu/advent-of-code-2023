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
    pub fn neighbor(&self, idx: usize) -> Option<Self> {
        let row = self.row as i64;
        let col = self.col as i64;

        let (row, col) = match idx {
            0 => (row - 1, col),
            1 => (row, col + 1),
            2 => (row + 1, col),
            3 => (row, col - 1),
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    location: Location,
    // NORTH, EAST, SOUTH, WEST
    connections: [bool; 4],
    c: char,
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
                c,
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
            c,
        })
    }

    pub fn neighbors<'a>(&self, nodes: &'a HashMap<Location, Node>) -> Vec<&'a Node> {
        let mut ret: Vec<&Node> = Vec::new();

        for i in 0..4 {
            if self.connections[i] {
                let l = self.location.neighbor(i);
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

pub fn bfs<'a>(nodes: &'a HashMap<Location, Node>, start: &'a Node) -> HashMap<Location, u64> {
    let mut stack: VecDeque<&Node> = VecDeque::new();
    let mut dists: HashMap<Location, u64> = HashMap::new();
    stack.push_back(start);
    dists.insert(start.location, 0);

    while !stack.is_empty() {
        let n = stack.pop_front().unwrap();
        let dist = dists[&n.location];

        for ni in n.neighbors(nodes) {
            if dists.contains_key(&ni.location) {
                continue;
            }

            dists.insert(ni.location, dist + 1);
            stack.push_back(&nodes[&ni.location]);
        }
    }

    dists
}
pub fn p2() -> std::io::Result<()> {
    let file = File::open("d10/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut nodes: HashMap<Location, Node> = HashMap::new();

    let mut start: Option<Node> = None;

    let mut cols = 0;
    let mut rows = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        let l = l?;
        cols = l.len();
        rows += 1;
        for (col, c) in l.chars().enumerate() {
            let loc = Location { row, col };
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

    println!("{:?}", start.neighbors(&nodes));

    let mut actual_conn = [false, false, false, false];
    for (idx, n) in start.neighbors(&nodes).iter().enumerate() {
        println!("{:?}", n);
        println!("{:?}", n.neighbors(&nodes));
        let valid = n
            .neighbors(&nodes)
            .iter()
            .any(|nn| nn.location == start.location);

        actual_conn[idx] = valid;
    }

    start.connections = actual_conn;

    println!("{:?}", start.connections);

    nodes.get_mut(&start.location).unwrap().connections = start.connections;

    let cycle = bfs(&nodes, &start);

    let mut contained: HashSet<Location> = HashSet::new();

    let mut imap: HashMap<Location, u64> = HashMap::new();

    for row in 0..rows {
        let mut intersections = 0;
        let mut last: Option<&Node> = None;
        let mut inside = false;

        for col in 0..cols {
            let location = Location { row, col };
            let node = cycle.get(&location).and(nodes.get(&location));

            match (last, node) {
                (None, Some(n)) => {
                    if n.connections[0] {
                        intersections += 1;
                    }
                }
                (None, None) => {}
                (Some(n), None) => {
                    // if n.connections[0]  {
                    //     intersections += 1;
                    // }
                }
                (Some(l), Some(n)) => {
                    if n.connections[0] {
                        intersections += 1;
                    }
                }
            }

            last = node;

            imap.insert(location, intersections);

            if cycle.get(&location).is_none() && intersections % 2 == 1 {
                contained.insert(location);
            }

            last = node;
        }
    }

    // for row in 0..rows {
    //     for col in 0..cols {
    //         let location = Location { row, col };
    //         if cycle.contains_key(&location) {
    //             print!("{}", nodes[&location].c);
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    // println!("------------------------");

    // for row in 0..rows {
    //     for col in 0..cols {
    //         let location = Location { row, col };
    //         print!("{:02}|", imap[&location]);
    //     }
    //     println!("");
    // }

    // println!("------------------------");

    // for row in 0..rows {
    //     for col in 0..cols {
    //         let l = Location { row, col };
    //         match contained.get(&l) {
    //             Some(_) => print!("X"),
    //             None => print!("."),
    //         }
    //     }
    //     println!("");
    // }

    println!("{}", contained.len());

    Ok(())
}

// Use a "ray" and count intersections, walking across each row
