use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn hash(input: &str) -> usize {
    let mut val: usize = 0;
    for c in input.chars() {
        val += c as usize;
        val *= 17;
        val = val % 256;
    }

    val
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d15/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut hmap: Vec<Vec<(String, u64)>> = Vec::new();
    for _ in 0..256 {
        hmap.push(Vec::new());
    }

    let file = File::open("d15/src/input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut s: String = String::new();
    buf_reader.read_line(&mut s);

    for step in s.split(',') {
        if step.contains('-') {
            let (label, _) = step.split_at(step.len() - 1);
            let h = hash(&label);
            let bucket = hmap.get_mut(h).unwrap();
            if let Some(idx) = bucket.iter().position(|(s, _)| s == label) {
                bucket.remove(idx);
            }
        }
        if step.contains("=") {
            let (label, flength) = step.split_once('=').unwrap();
            let flength = u64::from_str_radix(flength, 10).unwrap();

            let h = hash(&label);
            let bucket = hmap.get_mut(h).unwrap();
            let idx = bucket.iter().position(|(s, _)| s == label);
            match idx {
                Some(idx) => *bucket.get_mut(idx).unwrap() = (label.to_string(), flength),
                None => bucket.push((label.to_string(), flength)),
            }
        }
    }

    let mut sum = 0;

    for (idx, b) in hmap.iter().enumerate() {
        for (lidx, l) in b.iter().enumerate() {
            sum += ((1 + idx) * (lidx + 1) * l.1 as usize);
        }
    }

    println!("{sum}");

    Ok(())
}
