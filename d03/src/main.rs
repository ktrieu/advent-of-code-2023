use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    line: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Symbol {
    is_gear: bool,
    numbers: Vec<u32>,
}

fn add_number_if_exists(
    symbols: &mut HashMap<Position, Symbol>,
    line: usize,
    col: usize,
    number: u32,
) {
    match symbols.get_mut(&Position { line, col }) {
        Some(symbol) => symbol.numbers.push(number),
        None => {}
    };
}

fn record_number(
    symbols: &mut HashMap<Position, Symbol>,
    line_num: usize,
    start: usize,
    end: usize,
    number: u32,
) {
    let left_search_start = match start {
        0 => 0,
        _ => start - 1,
    };

    // First, check the same line
    add_number_if_exists(symbols, line_num, left_search_start, number);
    add_number_if_exists(symbols, line_num, end + 1, number);

    // Above
    if line_num != 0 {
        for col in left_search_start..=end + 1 {
            add_number_if_exists(symbols, line_num - 1, col, number);
        }
    }

    // Below
    for col in left_search_start..=end + 1 {
        add_number_if_exists(symbols, line_num + 1, col, number);
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("d03/src/input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut symbols: HashMap<Position, Symbol> = HashMap::new();

    for (line_num, l) in buf_reader.by_ref().lines().enumerate() {
        let l = l?;
        for (col_num, c) in l.chars().enumerate() {
            match c {
                '0'..='9' => {}
                '.' => {}
                _ => {
                    symbols.insert(
                        Position {
                            line: line_num,
                            col: col_num,
                        },
                        Symbol {
                            is_gear: match c {
                                '*' => true,
                                _ => false,
                            },
                            numbers: vec![],
                        },
                    );
                }
            }
        }
    }

    buf_reader.seek(SeekFrom::Start(0))?;

    let number_re = Regex::new("[0-9]+").unwrap();

    for (line_num, l) in buf_reader.by_ref().lines().enumerate() {
        let l = l?;

        println!("{line_num}");

        for m in number_re.find_iter(&l) {
            let number = u32::from_str_radix(m.as_str(), 10).unwrap();
            record_number(&mut symbols, line_num, m.start(), m.end() - 1, number);
        }
    }

    let mut sum = 0;

    for symbol in symbols.values() {
        if symbol.is_gear && symbol.numbers.len() == 2 {
            sum += symbol.numbers[0] * symbol.numbers[1];
        }
    }

    println!("{sum}");

    Ok(())
}
