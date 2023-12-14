use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    os,
    process::exit,
};

fn north(b: &mut HashMap<(usize, usize), char>, rocks: &Vec<(usize, usize)>, len: i64) -> i64 {
    let mut sum = 0;

    for (row, col) in rocks {
        if *row == 0 {
            sum += len;
            continue;
        }
        let mut next_idx: i64 = (row - 1) as i64;
        let mut next = *b.get(&(next_idx as usize, *col)).unwrap();

        while next != '#' && next != 'O' && next_idx >= 0 {
            next_idx -= 1;
            if next_idx < 0 {
                break;
            }
            next = *b.get(&(next_idx as usize, *col)).unwrap();
        }

        b.insert((*row, *col), '.');
        b.insert(((next_idx + 1) as usize, *col), 'O');
        sum += len - (next_idx + 1);
    }

    sum
}

fn east(b: &mut HashMap<(usize, usize), char>, rocks: &Vec<(usize, usize)>, len: i64) -> i64 {
    let mut sum = 0;

    for (row, col) in rocks.iter().rev() {
        if *col as i64 == len - 1 {
            sum += len as usize - row;
            continue;
        }
        let mut next_idx: i64 = (col + 1) as i64;
        let mut next = *b.get(&(*row, next_idx as usize)).unwrap();

        while next != '#' && next != 'O' && next_idx >= 0 {
            next_idx += 1;
            if next_idx >= len {
                break;
            }
            next = *b.get(&(*row, next_idx as usize)).unwrap();
        }

        b.insert((*row, *col), '.');
        b.insert((*row, (next_idx - 1) as usize), 'O');
        sum += len as usize - row;
    }

    sum as i64
}

fn west(b: &mut HashMap<(usize, usize), char>, rocks: &Vec<(usize, usize)>, len: i64) -> i64 {
    let mut sum = 0;

    for (row, col) in rocks {
        if *col == 0 {
            sum += len as usize - row;
            continue;
        }
        let mut next_idx: i64 = (col - 1) as i64;
        let mut next = *b.get(&(*row, next_idx as usize)).unwrap();

        while next != '#' && next != 'O' && next_idx >= 0 {
            next_idx -= 1;
            if next_idx < 0 {
                break;
            }
            next = *b.get(&(*row, next_idx as usize)).unwrap();
        }

        b.insert((*row, *col), '.');
        b.insert((*row, (next_idx + 1) as usize), 'O');
        sum += len as usize - row;
    }

    sum as i64
}

fn south(b: &mut HashMap<(usize, usize), char>, rocks: &Vec<(usize, usize)>, len: i64) -> i64 {
    let mut sum = 0;

    for (row, col) in rocks.iter().rev() {
        if *row == (len - 1) as usize {
            sum += 1;
            continue;
        }
        let mut next_idx: i64 = (row + 1) as i64;
        let mut next = *b.get(&(next_idx as usize, *col)).unwrap();

        while next != '#' && next != 'O' && next_idx >= 0 {
            next_idx += 1;
            if next_idx >= len {
                break;
            }
            next = *b.get(&(next_idx as usize, *col)).unwrap();
        }

        b.insert((*row, *col), '.');
        b.insert(((next_idx - 1) as usize, *col), 'O');
        sum += len - (next_idx - 1);
    }

    sum
}

fn grocks(b: &HashMap<(usize, usize), char>) -> Vec<(usize, usize)> {
    let mut vec: Vec<(usize, usize)> = Vec::new();

    for (k, v) in b {
        if *v == 'O' {
            vec.push(*k);
        }
    }

    vec.sort();

    vec
}

fn disp(b: &HashMap<(usize, usize), char>, len: i64) {
    let len = len as usize;
    for row in 0..len {
        for col in 0..len {
            print!("{}", b.get(&(row, col)).unwrap());
        }
        println!("");
    }
}

fn drocks(rocks: &Vec<(usize, usize)>) {
    for r in rocks {
        print!("[{}, {}] ", r.0, r.1);
    }
}

fn bstate(b: &HashMap<(usize, usize), char>, len: i64) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = Vec::new();
    let len = len as usize;
    for row in 0..len {
        v.push(Vec::new());
        for col in 0..len {
            v[row].push(*b.get(&(row, col)).unwrap());
        }
    }

    v
}

fn score(b: &HashMap<(usize, usize), char>, len: i64) -> usize {
    let mut sum = 0;

    for (k, v) in b {
        if *v == 'O' {
            let row = k.0;
            sum += len as usize - row;
            let check = (k.0, k.1 + 1);
            match b.get(&check) {
                Some(c) => {
                    if *c == '.' {
                        println!("{check:?}");
                        disp(&b, len);
                        exit(0);
                    }
                }
                None => {}
            }
        }
    }

    sum
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d14/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut b: HashMap<(usize, usize), char> = HashMap::new();

    let mut rocks: Vec<(usize, usize)> = Vec::new();

    let mut len = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        let l = l?;

        for (col, c) in l.chars().enumerate() {
            b.insert((row, col), c);
        }
        len += 1;
    }

    let mut sum = 0;
    let mut current_cycle = 0;

    let mut states: HashMap<Vec<Vec<char>>, u64> = HashMap::new();
    states.insert(bstate(&b, len), 0);

    let mut sum = 0;
    let bounds = (111, 111);

    for _ in 0..=1000 {
        rocks = grocks(&b);
        north(&mut b, &rocks, len);
        // println!("NORTH");
        // disp(&b, len);
        rocks = grocks(&b);
        west(&mut b, &rocks, len);
        // println!("WEST");
        // disp(&b, len);
        rocks = grocks(&b);
        south(&mut b, &rocks, len);
        // println!("SOUTH");
        // disp(&b, len);
        rocks = grocks(&b);
        east(&mut b, &rocks, len);
        // println!("EAST");
        // disp(&b, len);

        let s = score(&b, len);
        println!("ITER: {current_cycle} ({s})");

        if current_cycle >= bounds.0 && current_cycle <= bounds.1 {
            sum += s;
        }

        let state = bstate(&b, len);
        if let Some(c) = states.get(&state) {
            println!("CYCLE: {current_cycle}, starting at {c}");
            break;
        }
        states.insert(state, current_cycle);
        current_cycle += 1;
    }

    let mut vec: Vec<u64> = Vec::new();
    vec.extend(0..=107);
    while vec.len() < 1000000000 {
        vec.extend(108..=143)
    }

    println!("{}", vec[1000000000 - 1]);

    Ok(())
}
