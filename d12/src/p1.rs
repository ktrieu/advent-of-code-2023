use std::{
    collections::HashMap,
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

fn eval_slow(v: &[Spring]) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();

    let mut group = 0;
    let mut last = Spring::Working;

    for s in v {
        if *s == Spring::Broken {
            group += 1
        };

        if last == Spring::Broken && *s == Spring::Working {
            ret.push(group);
            group = 0;
        }

        last = *s;
    }

    ret.push(group);

    ret
}

fn eval(v: &Vec<Spring>, memo: &mut HashMap<Vec<Spring>, Vec<u64>>) -> Vec<u64> {
    let (last, rest) = v.split_last().unwrap();

    let sl = match rest.last() {
        Some(sl) => sl,
        None => {
            let res = if *last == Spring::Broken {
                vec![1]
            } else {
                vec![]
            };

            memo.insert(v.to_vec(), res.clone());
            return res;
        }
    };

    let result = match memo.get(&rest.to_vec()) {
        Some(b) => match last {
            Spring::Working => b.clone(),
            Spring::Broken => {
                let mut b = b.clone();
                match sl {
                    Spring::Working => {
                        b.push(1);
                    }
                    Spring::Broken => {
                        *b.last_mut().unwrap() += 1;
                    }
                    Spring::Unknown => unreachable!(),
                };
                b
            }
            Spring::Unknown => unreachable!(),
        },
        None => eval_slow(v),
    };

    memo.insert(v.to_vec(), result.clone());

    result.clone()
}

fn h(cond: &Vec<u64>, other: &Vec<u64>) -> bool {
    // dbg!(&cond);
    // dbg!(&other);

    if other.is_empty() {
        // dbg!(true);
        return true;
    }

    if other.len() > cond.len() {
        return false;
    }

    let mut res = true;

    for (idx, o) in other.iter().enumerate() {
        if cond[idx] != *o && !(idx == other.len() - 1 && *o <= cond[idx]) {
            res = false;
        }
    }

    // dbg!(res);

    return res;
}

fn solve2(
    original: &Vec<Spring>,
    current: Vec<Spring>,
    i: usize,
    l: u64,
    cond: &Vec<u64>,
    sols: &mut u64,
    memo: &mut HashMap<Vec<Spring>, Vec<u64>>,
) {
    // dbg!(&current);
    // dbg!(l);

    if i == original.len() {
        if eval(&current, memo) == *cond {
            *sols += 1;
        }
        return;
    }

    let c = original[i];
    // dbg!(&c);

    match c {
        Spring::Working => {
            let mut n = current.clone();
            n.push(Spring::Working);
            // dbg!(&n);
            let eval = eval(&n, memo);
            // dbg!(&eval);
            if h(&cond, &eval) {
                solve2(original, n, i + 1, l, cond, sols, memo);
            }
        }
        Spring::Broken => {
            let mut n = current.clone();
            n.push(Spring::Broken);
            // dbg!(&n);
            let eval = eval(&n, memo);
            // dbg!(&eval);
            if h(&cond, &eval) {
                solve2(original, n, i + 1, l, cond, sols, memo)
            }
        }
        Spring::Unknown => {
            let mut nw = current.clone();
            nw.push(Spring::Working);
            // dbg!(&nw);
            let evalw = eval(&nw, memo);
            // dbg!(&evalw);
            let working = if h(cond, &evalw) {
                solve2(original, nw, i + 1, l, cond, sols, memo)
            };

            let mut nb = current.clone();
            nb.push(Spring::Broken);
            // dbg!(&nb);
            let evalb = eval(&nb, memo);
            // dbg!(&evalb);
            let valid = h(cond, &evalb);
            let broken = if valid {
                solve2(original, nb, i + 1, l, cond, sols, memo)
            };
        }
    }
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d12/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut memo: HashMap<Vec<Spring>, Vec<u64>> = HashMap::new();

    let mut sum = 0;

    for l in buf_reader.lines() {
        let l = l?;

        let (springs_str, cond_str) = l.split_once(' ').unwrap();

        let springs: Vec<Spring> = springs_str.chars().map(|c| Spring::new(c)).collect();
        let cond: Vec<u64> = cond_str
            .split(",")
            .map(|s| u64::from_str_radix(s, 10).unwrap())
            .collect();

        let mut sols = 0;

        solve2(&springs, Vec::new(), 0, 0, &cond, &mut sols, &mut memo);
        sum += sols;
    }

    println!("{sum}");

    Ok(())
}
