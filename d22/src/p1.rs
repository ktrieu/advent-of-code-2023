use std::{
    fs::File,
    io::{BufRead, BufReader}, ops, fmt::Display, collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64
}

impl Coord {
    pub fn new(s: &str) -> Self {
        let coords: Vec<i64> = s.split(",").map(|c| i64::from_str_radix(c, 10).unwrap()).collect();
        assert!(coords.len() == 3);

        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2]
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs()
        }
    }

    pub fn clamp(&self, c: i64) -> Self {
        let c = c.abs();
        Self {
            x: self.x.clamp(-c, c),
            y: self.y.clamp(-c, c),
            z: self.z.clamp(-c, c)
        }
    }

    pub fn in_world(&self) -> bool {
        self.z >= 0
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = *self + rhs
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

struct BlockPosIterator {
    current: Coord,
    end: Coord,
    dir: Coord
}

impl BlockPosIterator {
    pub fn new(b: &Block) -> Self {
        Self {
            current: b.c1,
            end: b.c2 + b.dir,
            dir: b.dir
        }
    }
}

impl Iterator for BlockPosIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != self.end {
            let ret = Some(self.current);
            self.current += self.dir;
            ret
        }
        else {
            None
        }
    }
}
#[derive(Clone, Copy,)]
struct Block {
    c1: Coord,
    c2: Coord,
    dir: Coord,
    id: usize,
}

const ZERO: Coord = Coord {
    x: 0,
    y: 0,
    z: 0,
};

impl Block {
    pub fn new(s: &str, id: usize) -> Self {
        let (c1, c2) = s.split_once("~").unwrap();
        let c1 = Coord::new(c1);
        let c2 = Coord::new(c2);

        let delta = c2 - c1;

        let mut dir = delta.clamp(1);
        
        // Assign an arbitrary direction to one cube blocks
        if delta == ZERO {
            dir = Coord { x: 1, y: 0, z: 0 };
        }

        Self {
            c1,
            c2,
            dir,
            id
        }
    }

    pub fn coords(&self) -> BlockPosIterator {
        BlockPosIterator::new(self)
    }

    pub fn min_z(&self) -> i64 {
        if self.dir.z == -1 {
            self.c2.z
        }
        else {
            self.c1.z
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {} - {} [dir: {}]", self.id, self.c1, self.c2, self.dir)
    }
}

struct World {
    blocks: Vec<Block>,
    occupancy: HashMap<Coord, usize>,
    support: HashMap<usize, HashSet<usize>>,
}

const DOWN: Coord = Coord {
    x: 0,
    y: 0,
    z: -1
};

impl World {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            occupancy: HashMap::new(),
            support: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, s: &str) {
        let id = self.blocks.len();
        let b = Block::new(s, id);

        for c in b.coords() {
            self.occupancy.insert(c, id);
        }

        self.blocks.push(b);
    }

    pub fn move_block(&mut self, idx: usize, delta: Coord) {
        let b = self.blocks.get(idx).unwrap();
        let mut moved = *b;

        moved.c1 += delta;
        moved.c2 += delta;
    
        // Recalculate occupancy
        for c in b.coords() {
            self.occupancy.remove(&c);
        }

        for c in moved.coords() {
            let present = self.occupancy.insert(c, idx).is_some();
            // Do not move a block where one already exists!
            assert!(!present);
        }

        *self.blocks.get_mut(idx).unwrap() = moved;
    }

    pub fn sim_block(&mut self, idx: usize) {
        let b = self.blocks.get(idx).unwrap();
        let mut coords: Vec<Coord> = b.coords().collect();

        let mut dist = 0;
        'outer: loop {  
            for c in &mut coords {
                *c += DOWN;
                if !c.in_world() {
                    break 'outer;
                }
                if let Some(other) = self.occupancy.get(&c) {
                    // Self-collisions don't count
                    if *other != idx {
                        break 'outer;
                    }
                        
                }            
            }
            dist += 1;
        }

        // Optimization: don't bother editing anything if we haven't moved
        if dist == 0 {
            return
        }

        self.move_block(idx, Coord { x: 0, y: 0, z: -dist} );
    }

    pub fn sim(&mut self) {
        // Sort bottom up so we simulate properly
        let mut sorted = self.blocks.clone();
        sorted.sort_by_key(|b| b.min_z());

        for s in sorted {
            self.sim_block(s.id);
        }
    }

    pub fn calculate_support(&mut self) {
        for b in &self.blocks {
            // println!("{b}");
            let mut support: HashSet<usize> = HashSet::new();
            for c in b.coords() {
                let under = c + DOWN;
                // println!("{under}");
                if let Some(under_block) = self.occupancy.get(&under) {
                    // println!("Found supporting block for {}: {}", b.id, under_block);
                    if *under_block != b.id {
                        support.insert(*under_block);
                    }
                }
            }
            self.support.insert(b.id, support);
        }

    }
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d22/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut w = World::new();

    for l in buf_reader.lines() {
        w.add_block(&l?);
    }

    w.sim();
    w.calculate_support();

    let mut required: HashSet<usize> = HashSet::new();

    // println!("{:?}", &w.support);
    for (_, v) in w.support {
        if v.len() == 1 {
            required.insert(*v.iter().next().unwrap());
        }
    }

    println!("{}", w.blocks.len() - required.len());

    Ok(())
}
