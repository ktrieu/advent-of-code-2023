use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Read},
};

struct Card {
    id: u32,
    winning: HashSet<u32>,
    actual: HashSet<u32>,
}

impl Card {
    fn parse_numbers(s: &str) -> HashSet<u32> {
        println!("{s}");
        s.split_whitespace()
            .map(|c| u32::from_str_radix(c.trim(), 10).unwrap())
            .collect()
    }

    pub fn new(line: &str) -> Self {
        let (_, rest) = line.split_once(':').unwrap();

        let (winning_str, actual_str) = rest.trim().split_once('|').unwrap();

        let winning = Self::parse_numbers(winning_str);
        let actual = Self::parse_numbers(actual_str);

        Self {
            id: 0,
            winning,
            actual,
        }
    }

    pub fn matches(&self) -> u32 {
        self.winning.intersection(&self.actual).count() as u32
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("d04/src/input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut cards: Vec<Card> = Vec::new();

    let mut counts: Vec<u32> = Vec::new();

    for (line_num, l) in buf_reader.by_ref().lines().enumerate() {
        let l = l?;
        let card = Card::new(&l);

        cards.push(card);
        counts.push(1);
    }

    for (idx, card) in cards.iter().enumerate() {
        let matches = card.matches();

        for next_count in 1..=matches {
            let next_idx = idx as u32 + next_count;
            if next_idx as usize >= cards.len() {
                break;
            }
            counts[next_idx as usize] += counts[idx];
        }
    }

    let score: u32 = counts.iter().sum();
    println!("{score}");

    Ok(())
}
