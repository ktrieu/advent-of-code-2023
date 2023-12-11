use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn new(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

type NodeName = String;
pub struct Node {
    name: NodeName,
    left: NodeName,
    right: NodeName,
}

impl Node {
    pub fn new(l: &str) -> Self {
        let regex =
            Regex::new(r"([A-Z][A-Z][A-Z]) = \(([A-Z][A-Z][A-Z]), ([A-Z][A-Z][A-Z])\)").unwrap();

        let m = regex.captures(l).unwrap();

        let name = m.get(1).unwrap().as_str().to_string();
        let left = m.get(2).unwrap().as_str().to_string();
        let right = m.get(3).unwrap().as_str().to_string();

        return Self { name, left, right };
    }
}

pub fn route(
    n: &NodeName,
    nodes: &HashMap<NodeName, Node>,
    instructions: &Vec<Direction>,
) -> usize {
    let mut current = n;
    let mut steps = 0;

    for inst in instructions.iter().cycle() {
        if current.ends_with('Z') {
            break;
        }

        let next = match inst {
            Direction::Left => &nodes.get(current).unwrap().left,
            Direction::Right => &nodes.get(current).unwrap().right,
        };

        current = next;

        steps += 1;
    }

    steps
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d08/src/input.txt")?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let mut nodes: HashMap<NodeName, Node> = HashMap::new();

    let mut first: Option<NodeName> = None;

    let instructions: Vec<Direction> = lines
        .next()
        .unwrap()?
        .chars()
        .map(|c| Direction::new(c))
        .collect();

    lines.next();

    for l in lines {
        let l = l?;

        let node = Node::new(&l);
        if first.is_none() {
            first = Some(node.name.clone());
        }
        nodes.insert(node.name.clone(), node);
    }

    let mut steps = 0;

    let a_nodes: Vec<usize> = nodes
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|n| route(n, &nodes, &instructions))
        .collect();

    println!("{a_nodes:?}");

    Ok(())
}
