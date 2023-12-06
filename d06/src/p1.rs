use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_line(line: &str) -> Vec<i64> {
    let (_, rest) = line.split_once(':').unwrap();

    rest.split_whitespace()
        .map(|s| i64::from_str_radix(s, 10).unwrap())
        .collect()
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn wins_possible(&self) -> usize {
        let det = f64::sqrt((self.time.pow(2) - (4 * self.distance)) as f64);

        let r1 = (-self.time as f64 + det) / -2_f64;
        let r2 = (-self.time as f64 - det) / -2_f64;

        let first = r1.ceil() as i64;
        let last = r2.floor() as i64;

        println!("{first} {last}");

        return (first..=last).count();
    }
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d06/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();
    let time_line = lines.next().unwrap()?;
    let dist_line = lines.next().unwrap()?;

    let times = parse_line(&time_line);
    let dists = parse_line(&dist_line);

    let races: Vec<Race> = times
        .iter()
        .zip(dists.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect();

    let sum: usize = races
        .iter()
        .map(|r| r.wins_possible())
        .inspect(|w| println!("{w}"))
        .product();

    println!("{sum}");

    Ok(())
}
