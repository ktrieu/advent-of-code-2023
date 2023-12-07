use std::{collections::HashMap, io::{BufReader, BufRead}, fs::File, cmp::Ordering, ops::Index, fmt::Display};

#[derive(Debug)]
struct Hand {
    cards_ordered: Vec<char>,
    cards: HashMap<char, usize>,
    bid: u64
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.cards_ordered.iter().collect();
        let ty = self.hand_type();

        let tys = match ty {
          6 => "five of a kind",
          5 => "four of a kind",
          4 => "full house",
          3 => "three of a kind",
          2 => "two pair",
          1 => "one pair",
          0 => "high card",
          _ => "uh oh"
        };
        write!(f, "{s} ({tys})")
    }
}

const CARD_ORDER: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn cards_cmp(lhs: char, rhs: char) -> Ordering {     
    CARD_ORDER.iter().position(|c| *c == lhs).unwrap().cmp(&CARD_ORDER.iter().position(|c| *c == rhs).unwrap())
}

impl Hand {
    pub fn new(line: &str) -> Self {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();

        let mut cards: HashMap<char, usize> = HashMap::new();
        let mut cards_ordered: Vec<char> = Vec::new();

        for c in cards_str.chars() {
            cards_ordered.push(c);
            
            if !cards.contains_key(&c) {
                cards.insert(c, 0);
            }

            cards.insert(c, cards.get(&c).unwrap() + 1);
        }

        let bid = u64::from_str_radix(&bid_str, 10).unwrap();

        Self {
            cards,
            cards_ordered,
            bid
        }
    }

    pub fn hand_type(&self) -> u64 {
        let jokered = self.jokerfy();

        if jokered.values().any(|n| *n == 5) {
            return 6
        }

        if jokered.values().any(|n| *n == 4) {
            return 5
        }

        if jokered.values().filter(|n| **n == 3).count() == 1 && jokered.values().filter(|n| **n == 2).count() == 1 {
            return 4
        }

        if jokered.values().any(|n| *n == 3) {
            return 3
        }

        if jokered.values().filter(|n| **n == 2).count() == 2 {
            return 2
        }

        if jokered.values().filter(|n| **n == 2).count() == 1 {
            return 1
        }

        0
    }

    pub fn jokerfy(&self) -> HashMap<char, usize> {
        let mut jokered = self.cards.clone();
        if jokered.contains_key(&'J') {
            let num_jokers = *jokered.get(&'J').unwrap();
            let highest_other = jokered.iter().filter(|(k, _)| **k != 'J').max_by_key(|(_, v)| *v);
            if highest_other.is_some() {
                jokered.insert(*highest_other.unwrap().0, jokered.get(highest_other.unwrap().0).unwrap() + num_jokers);
                jokered.remove(&'J');
            }
        }

        jokered
    }

    pub fn rank(&self, other: &Hand) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        if self_type < other_type {
            return Ordering::Less
        }
        else if self_type > other_type {
            return Ordering::Greater
        }
        else {
            for (sc, oc) in self.cards_ordered.iter().zip(other.cards_ordered.iter()) {
                let ordering = cards_cmp(*sc, *oc);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }

        Ordering::Equal
    }
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d07/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut hands: Vec<Hand> = buf_reader.lines().map(|l| Hand::new(&l.unwrap())).collect();

    hands.sort_by(|lhs, rhs| lhs.rank(rhs));

    let mut sum = 0;

    for (idx, c) in hands.iter().enumerate() {
        let rank = idx + 1;
        println!("{rank} {c}");
        sum += rank * c.bid as usize;
    }

    println!("{sum}");

    Ok(())
}