use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_line(line: &str) -> i64 {
    let (_, rest) = line.split_once(':').unwrap();

    let no_whitespace: String = rest.split_whitespace().collect();

    i64::from_str_radix(&no_whitespace, 10).unwrap()
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

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d06/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();
    let time_line = lines.next().unwrap()?;
    let dist_line = lines.next().unwrap()?;

    let time = parse_line(&time_line);
    let dist = parse_line(&dist_line);

    let race = Race {
        time,
        distance: dist,
    };

    let v = race.wins_possible();

    println!("{v}");

    Ok(())
}
