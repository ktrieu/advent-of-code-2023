use std::{fs::File, io::{BufReader, BufRead}};

pub struct Sequence {
    sequences: Vec<Vec<i64>>
}

impl Sequence {
    pub fn new(l: &str) -> Self {
        let first: Vec<i64> = l.split_whitespace().map(|s| i64::from_str_radix(s, 10).unwrap()).collect();

        Self {
            sequences: vec![first]
        }
    }

    pub fn diff(&mut self) -> bool {
        let mut diffed: Vec<i64> = Vec::new();
        let last = &self.sequences.last().unwrap();

        for i in 0..last.len() - 1 {
            let (a, b) = (last[i], last[i + 1]);
            let diff = b - a;
            diffed.push(diff)
        }
        let done = diffed.iter().all(|i| *i == 0);

        self.sequences.push(diffed);

        done

    }

    pub fn extrap(&self) -> i64 {
        let mut val = 0;

        for s in self.sequences.iter().rev() {
            println!("{val}");
            val = s.first().unwrap() - val;
        }

        val
    }
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d09/src/justin.txt")?;
    let buf_reader = BufReader::new(file);

    let mut sum = 0;

    for l in buf_reader.lines() {
        let l = l?;
        let mut s = Sequence::new(&l);

        let mut d = false;
        while !d {
            d = s.diff();
        }

        let v = s.extrap();
        sum += v;
    }

    println!("{sum}");

    Ok(())
}