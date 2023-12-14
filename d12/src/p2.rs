use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl Spring {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Working,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Working => write!(f, "."),
            Spring::Broken => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct SolveInput {
    pos: usize,
    // Our current group.
    current: Option<usize>,
    // Remaining groups to fulfill, reversed for efficiency.
    remaining: Vec<usize>,
}

impl SolveInput {
    pub fn exit_group(&self) -> Option<Self> {
        println!("EXIT");
        match self.current {
            Some(cgroup) => {
                if cgroup != *self.remaining.last().unwrap() {
                    None
                } else {
                    let mut c = self.clone();
                    c.current = None;
                    c.remaining.pop().unwrap();
                    c.pos += 1;
                    Some(c)
                }
            }
            None => unreachable!(),
        }
    }

    pub fn inc_group(&self) -> Option<Self> {
        println!("INC");
        match self.current {
            Some(cgroup) => {
                if cgroup == *self.remaining.last().unwrap() {
                    None
                } else {
                    let mut c = self.clone();
                    c.current = Some(cgroup + 1);
                    c.pos += 1;
                    Some(c)
                }
            }
            None => unreachable!(),
        }
    }

    pub fn enter_group(&self) -> Option<Self> {
        println!("ENTER");
        match self.current {
            Some(_) => unreachable!(),
            None => {
                if self.remaining.len() == 0 {
                    return None;
                }
                let mut c = self.clone();
                c.current = Some(1);
                c.pos += 1;

                Some(c)
            }
        }
    }

    pub fn is_in_group(&self) -> bool {
        self.current.is_some()
    }
}

impl Display for SolveInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current = self
            .current
            .map(|c| c.to_string())
            .unwrap_or("X".to_string());
        let remaining = self
            .remaining
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "position: {}, current: {}, remaining: {}",
            self.pos, current, remaining
        )
    }
}

fn solve(original: &Vec<Spring>, input: SolveInput) -> usize {
    println!("{input}");

    if input.pos == original.len() {
        // let sols = match (input.remaining.last(), input.current) {
        //     (Some(remaining), Some(current)) => {
        //         if input.remaining.len() == 1 && *remaining == current {
        //             println!("SOLUTION");
        //             1
        //         } else {
        //             println!("BRANCH FAIL");
        //             0
        //         }
        //     }
        //     _ => {
        //         println!("BRANCH FAIL");
        //         0
        //     }
        // };

        let input = if input.is_in_group() {
            input.exit_group()
        } else {
            Some(input)
        };

        let sols = if input.is_some() && input.unwrap().remaining.len() == 0 {
            println!("SOLUTION");
            1
        } else {
            println!("BRANCH FAIL");
            0
        };

        return sols;
    };

    let next = original[input.pos];
    println!("{next}");

    let sols = match (next, input.is_in_group()) {
        (Spring::Working, true) => {
            let n = input.exit_group();
            match n {
                Some(n) => solve(&original, n),
                None => 0,
            }
        }
        (Spring::Working, false) => {
            let mut n = input.clone();
            n.pos += 1;
            solve(original, n)
        }
        (Spring::Broken, true) => {
            let n = input.inc_group();
            match n {
                Some(n) => solve(&original, n),
                None => 0,
            }
        }
        (Spring::Broken, false) => {
            let n = input.enter_group();
            match n {
                Some(n) => solve(&original, n),
                None => 0,
            }
        }
        (Spring::Unknown, true) => {
            println!("WORKING: {}", input.pos);
            let working_input = input.exit_group();
            let working = match working_input {
                Some(n) => solve(original, n),
                None => 0,
            };

            println!("BROKEN: {}", input.pos);
            let broken_input = input.inc_group();
            let broken = match broken_input {
                Some(n) => solve(original, n),
                None => 0,
            };

            working + broken
        }
        (Spring::Unknown, false) => {
            println!("BROKEN: {}", input.pos);
            let broken_input = input.enter_group();
            let broken = match broken_input {
                Some(n) => solve(original, n),
                None => 0,
            };

            println!("WORKING: {}", input.pos);
            let mut working_input = input.clone();
            working_input.pos += 1;
            let working = solve(original, working_input);

            working + broken
        }
    };

    sols
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d12/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut sum = 0;

    for (idx, l) in buf_reader.lines().enumerate() {
        let l = l?;

        let (springs_str, cond_str) = l.split_once(' ').unwrap();

        let springs: Vec<Spring> = springs_str.chars().map(|c| Spring::new(c)).collect();
        // let mut springs_d: Vec<Spring> = Vec::new();
        // springs_d.extend(&springs);
        // springs_d.push(Spring::Unknown);
        // springs_d.extend(&springs);
        // springs_d.push(Spring::Unknown);
        // springs_d.extend(&springs);
        // springs_d.push(Spring::Unknown);
        // springs_d.extend(&springs);
        // springs_d.push(Spring::Unknown);
        // springs_d.extend(&springs);

        let mut cond: Vec<usize> = cond_str
            .split(",")
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect();

        // let len = cond.len();
        // let mut cond: Vec<usize> = cond.into_iter().cycle().take(len * 5).collect();
        cond.reverse();

        let res = solve(
            &springs,
            SolveInput {
                pos: 0,
                current: None,
                remaining: cond,
            },
        );
        sum += res;

        println!("Completed {idx}. Result: {res}");
        println!("------------------------------")
    }

    println!("{sum}");

    Ok(())
}
