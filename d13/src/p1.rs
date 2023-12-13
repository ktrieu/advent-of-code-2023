use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn refl_inner(b: &HashMap<Vec<char>, Vec<usize>>, d: &Vec<Vec<char>>, i: usize) -> bool {
    let mut l = i;
    let mut r = i + 1;

    let mut checked = false;

    // dbg!(i);

    while l >= 0 && r < d.len() {
        // dbg!(l);
        // dbg!(r);
        let v = &d[l];
        let occ = &b[v];

        // dbg!(v);
        // dbg!(occ);

        if !(occ.contains(&l) && occ.contains(&r)) {
            return false;
        }

        if l == 0 {
            return true;
        }
        l -= 1;
        r += 1;
    }

    true
}

fn refl(b: &HashMap<Vec<char>, Vec<usize>>, d: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..d.len() - 1 {
        if refl_inner(b, d, i) {
            return Some(i);
        }
    }

    None
}

fn pattern(reader: &mut BufReader<File>) -> Option<usize> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut cols: Vec<Vec<char>> = Vec::new();

    for (idx, l) in reader
        .lines()
        .take_while(|l| !l.as_ref().unwrap().is_empty())
        .enumerate()
    {
        let l = l.unwrap();
        rows.push(l.chars().collect());

        for (idx, c) in l.chars().enumerate() {
            if idx >= cols.len() {
                cols.push(Vec::new());
            }

            cols[idx].push(c);
        }
    }

    for r in &rows {
        println!("{}", r.iter().collect::<String>());
    }

    if rows.len() == 0 {
        return None;
    }

    let mut rowb: HashMap<Vec<char>, Vec<usize>> = HashMap::new();
    for (idx, r) in rows.iter().enumerate() {
        if !rowb.contains_key(r) {
            rowb.insert(r.clone(), Vec::new());
        }

        rowb.get_mut(r).unwrap().push(idx);
    }

    let mut colb: HashMap<Vec<char>, Vec<usize>> = HashMap::new();
    for (idx, r) in cols.iter().enumerate() {
        if !colb.contains_key(r) {
            colb.insert(r.clone(), Vec::new());
        }

        colb.get_mut(r).unwrap().push(idx);
    }

    let mut sum = 0;

    match refl(&rowb, &rows) {
        Some(r) => {
            println!("row: {r}");
            sum += 100 * (r + 1)
        }
        None => {}
    }

    match refl(&colb, &cols) {
        Some(r) => {
            println!("col: {r}");
            sum += r + 1
        }
        None => {}
    }

    return Some(sum);
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d13/src/input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut sum = 0;
    while let Some(s) = pattern(&mut buf_reader) {
        sum += s;
        println!("");
    }

    println!("{sum}");

    Ok(())
}
