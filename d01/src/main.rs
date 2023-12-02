use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aho_corasick::{AhoCorasick, Match};

const PATTERNS: &[&str; 18] = &[
    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
    "eight", "8", "nine", "9",
];

fn pattern_idx_to_digit(pattern_id: u32) -> u32 {
    (pattern_id / 2) + 1
}

fn main() -> std::io::Result<()> {
    let file = File::open("d01/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let matcher = AhoCorasick::new(PATTERNS).unwrap();

    let mut sum = 0;

    for l in buf_reader.lines() {
        let l = l?;
        let matches: Vec<Match> = matcher.find_overlapping_iter(&l).collect();

        let first_pattern_id = matches.first().unwrap().pattern().as_u32();
        let last_pattern_id = matches.last().unwrap().pattern().as_u32();

        let first_digit = pattern_idx_to_digit(first_pattern_id);
        let last_digit = pattern_idx_to_digit(last_pattern_id);

        sum += (10 * first_digit) + last_digit;
    }

    println!("{sum}");

    Ok(())
}
