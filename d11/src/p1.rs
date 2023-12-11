use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn dist(a: (i64, i64), b: (i64, i64)) -> u64 {
    b.0.abs_diff(a.0) + b.1.abs_diff(a.1)
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d11/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    let mut colp: HashMap<i64, bool> = HashMap::new();
    let mut rowp: HashMap<i64, bool> = HashMap::new();

    let mut rows = 0;
    let mut cols = 0;

    for (row, l) in buf_reader.lines().enumerate() {
        let l = l?;
        cols = l.len();
        for (col, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    galaxies.push((row as i64, col as i64));
                    colp.insert(col as i64, true);
                    rowp.insert(row as i64, true);
                }
                _ => {}
            }
        }

        rows += 1;
    }

    let mut adjr: Vec<i64> = Vec::new();
    let mut adjc: Vec<i64> = Vec::new();

    let mut adj = 0;
    for row in 0..rows {
        if !rowp.contains_key(&(row as i64)) {
            adj += 1;
        }

        adjr.push(adj)
    }

    adj = 0;
    for col in 0..cols {
        if !colp.contains_key(&(col as i64)) {
            adj += 1;
        }

        adjc.push(adj)
    }

    for g in galaxies.iter_mut() {
        let row = g.0;
        let col = g.1;
        g.0 += adjr[row as usize];
        g.1 += adjc[col as usize];
    }

    println!("{:?} {:?}", galaxies[2], galaxies[5]);
    println!("{}", dist(galaxies[2], galaxies[5]));

    let mut sum = 0;

    for a in galaxies.iter() {
        for b in galaxies.iter() {
            sum += dist(*a, *b);
        }
    }

    sum /= 2;

    println!("{sum}");

    Ok(())
}
