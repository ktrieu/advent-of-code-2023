use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Balls {
    blue: u32,
    red: u32,
    green: u32,
}

impl Balls {
    pub fn from_trial(trial_str: &str) -> Self {
        let mut balls = Self {
            blue: 0,
            red: 0,
            green: 0,
        };

        let re = Regex::new(r"([0-9]*) (red|blue|green)").unwrap();

        for ball_str in trial_str.split(",") {
            let ball_str = ball_str.trim();
            let captures = re.captures(ball_str).unwrap();

            let count = u32::from_str_radix(captures.get(1).unwrap().as_str(), 10).unwrap();
            let color = captures.get(2).unwrap().as_str();

            match color {
                "blue" => balls.blue += count,
                "red" => balls.red += count,
                "green" => balls.green += count,
                _ => panic!("Invalid color {color}"),
            };
        }

        return balls;
    }
}

const INITIAL: Balls = Balls {
    blue: 14,
    red: 12,
    green: 13,
};

#[derive(Debug)]
struct Game {
    id: u32,
    trials: Vec<Balls>,
}

impl Game {
    fn parse_game_id(game_id: &str) -> u32 {
        let re = Regex::new(r"Game ([0-9]*)").unwrap();

        let id_group = re
            .captures(game_id)
            .expect("Invalid game ID format!")
            .get(1)
            .expect("No numeric game ID!");

        u32::from_str_radix(id_group.as_str(), 10).unwrap()
    }

    pub fn from_line(line: &str) -> Self {
        let (game_id_str, rest) = line.split_once(":").expect("No colon in game line!");

        let game_id = Self::parse_game_id(game_id_str);

        let trials: Vec<Balls> = rest
            .split(";")
            .map(|trial_str| Balls::from_trial(trial_str.trim()))
            .collect();

        Self {
            id: game_id,
            trials,
        }
    }
}

fn is_valid_trial(balls: &Balls) -> bool {
    if balls.red > INITIAL.red || balls.green > INITIAL.green || balls.blue > INITIAL.blue {
        return false;
    }

    true
}

fn get_game_power(game: &Game) -> u32 {
    let red_min = game.trials.iter().map(|t| t.red).max().unwrap();
    let blue_min = game.trials.iter().map(|t| t.blue).max().unwrap();
    let green_min = game.trials.iter().map(|t| t.green).max().unwrap();

    return red_min * blue_min * green_min;
}

fn main() -> std::io::Result<()> {
    let file = File::open("d02/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut sum = 0;

    for l in buf_reader.lines() {
        let l = l?;

        let game = Game::from_line(&l);

        sum += get_game_power(&game);
    }

    println!("{sum}");

    Ok(())
}
