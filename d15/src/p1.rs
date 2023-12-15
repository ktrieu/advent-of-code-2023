use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d15/src/input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut s: String = String::new();
    buf_reader.read_line(&mut s);

    let mut sum = 0;

    for step in s.split(',') {
        let mut val: u64 = 0;
        for c in step.chars() {
            val += c as u64;
            val *= 17;
            val = val % 256;
        }
        sum += val;
    }

    println!("{sum}");

    Ok(())
}
