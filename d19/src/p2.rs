use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d19/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    for l in buf_reader.lines() {}

    Ok(())
}
