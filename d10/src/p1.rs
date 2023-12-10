use std::{io::{BufReader, BufRead}, fs::File, collections::{HashMap, HashSet, VecDeque}};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Location {
    row: usize,
    col: usize,
}   

impl Location {
    pub fn neighbor(&self, idx: usize) -> Self {
        match idx {
            0 => Self { row: self.row - 1, col: self.col },
            1 => Self { row: self.row, col: self.col + 1},
            2 => Self { row: self.row + 1, col: self.col },
            3 => Self { row: self.row, col: self.col - 1, },
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
            connections
        })

    }

    pub fn neighbors<'a>(&self, nodes: &'a HashMap<Location, Node>) -> Vec<&'a Node> {
        let mut ret: Vec<&Node> = Vec::new();

        for i in 0..4 {
            if self.connections[i] {
                let l = self.location.neighbor(i);
                if nodes.contains_key(&l) {
                    ret.push(&nodes[&l]);
                }
            }
        };

        ret
    }
}

pub fn dist<'a>(nodes: &'a HashMap<Location, Node>, start: &'a Node) -> HashMap<Location, u64> {
    let mut stack: VecDeque<&Node> = VecDeque::new();
    let mut dists: HashMap<Location, u64> = HashMap::new();
    stack.push_back(start);
    dists.insert(start.location, 0);

    let mut m = 0;
    let mut mn: Option<Location> = None;

    while !stack.is_empty() {
        let n = stack.pop_front().unwrap();
        let dist = dists[&n.location];

        for ni in n.neighbors(nodes) {      
            if dists.contains_key(&ni.location) {
                continue
            }
    
            dists.insert(ni.location, dist + 1);

            if dist + 1 > m {
                mn = Some(ni.location);
                m = dist + 1;
            }


            stack.push_back(&nodes[&ni.location]);
        }
    }

    println!("{mn:?}");

    dists
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d10/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut nodes: HashMap<Location, Node> = HashMap::new();

    let mut start: Option<Node> = None;

    let mut rl = 0;
    let mut rows = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        rl = 0;
        rows += 1;
        for (col, c) in l?.chars().enumerate() {
            rl += 1;
            let loc = Location { row: row + 1, col: col + 1 };
            let node = Node::new(c, loc);
            match node {
                Some(node) => { 
                    if c == 'S' {
                        start = Some(node);
                    }
                    nodes.insert(loc, node);
                },
                None => {}
            }
        }
    };

    let mut start = start.unwrap();

    for (idx, n) in start.neighbors(&nodes).iter().enumerate() {
        if !n.neighbors(&nodes).iter().any(|n| n.location == start.location) {
            start.connections[idx] = false;
        }
    }

    let dists = dist(&nodes, &start);
    let max = dists.values().max().unwrap();

    for x in 0..rl {
        for y in 0..rows {
            if dists.contains_key(&Location { row: y + 1, col: x + 1}) {
                print!("{:?}", dists[&Location { row: y + 1, col: x + 1}]);
            }
            else {
                print!("----");
            }
        }
        println!("");
}

    println!("{max}");

    Ok(())
}