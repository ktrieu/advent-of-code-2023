use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn refl_inner(
    b: &HashMap<Vec<char>, Vec<usize>>,
    d: &Vec<Vec<char>>,
    leven: &HashMap<Vec<char>, Vec<Vec<char>>>,
    i: usize,
) -> bool {
    dbg!(i);

    let mut l = i;
    let mut r = i + 1;

    let mut swap_used = false;

    while l >= 0 && r < d.len() {
        dbg!(l);
        dbg!(r);
        let v = &d[l];
        let occ = &b[v];

        dbg!(&occ);

        if !(occ.contains(&l) && occ.contains(&r)) {
            if swap_used {
                return false;
            }

            if !swap_used {
                let cands = leven.get(v).unwrap();
                for c in cands {
                    println!(
                        "SWAP:\n{}\n{}",
                        &v.iter().collect::<String>(),
                        &c.iter().collect::<String>()
                    );
                    let occ_c = &b[c];
                    dbg!(occ_c);
                    if !(occ_c.contains(&r)) {
                        println!("SWAP FAILED");
                        continue;
                    };
                    println!("USED");
                    swap_used = true;
                    break;
                }
            }

            if !swap_used {
                println!("SWAP REALLY FAILED");
                return false;
            }
        }

        if l == 0 {
            return swap_used;
        }
        l -= 1;
        r += 1;
    }

    swap_used
}

fn refl(
    b: &HashMap<Vec<char>, Vec<usize>>,
    d: &Vec<Vec<char>>,
    leven: &HashMap<Vec<char>, Vec<Vec<char>>>,
) -> Option<usize> {
    for i in 0..d.len() - 1 {
        if refl_inner(b, d, leven, i) {
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

    let mut rowd: HashMap<Vec<char>, Vec<Vec<char>>> = HashMap::new();
    for a in rowb.keys() {
        rowd.insert(a.clone(), Vec::new());
        for b in rowb.keys() {
            if leven(&a, &b) {
                rowd.get_mut(a).unwrap().push(b.clone());
            }
        }
    }

    let mut cold: HashMap<Vec<char>, Vec<Vec<char>>> = HashMap::new();
    for a in colb.keys() {
        cold.insert(a.clone(), Vec::new());
        for b in colb.keys() {
            if leven(&a, &b) {
                cold.get_mut(a).unwrap().push(b.clone());
            }
        }
    }

    let mut sum = 0;

    let mut added = false;

    match refl(&rowb, &rows, &rowd) {
        Some(r) => {
            println!("row: {r}");
            added = true;
            sum += 100 * (r + 1)
        }
        None => {}
    }

    if !added {
        match refl(&colb, &cols, &cold) {
            Some(r) => {
                println!("col: {r}");
                added = true;
                sum += r + 1
            }
            None => {}
        }
    }

    for r in &rows {
        println!("{}", r.iter().collect::<String>());
    }

    return Some(sum);
}

fn leven(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut swap_used = false;

    for (ca, cb) in zip(a.iter(), b.iter()) {
        if ca != cb {
            if !swap_used {
                swap_used = true
            } else {
                return false;
            }
        }
    }

    swap_used
}

pub fn p2() -> std::io::Result<()> {
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
