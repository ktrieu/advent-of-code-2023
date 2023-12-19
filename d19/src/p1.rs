use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    pub fn new(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Lt,
    Gt,
}

impl Op {
    pub fn new(c: char) -> Self {
        match c {
            '<' => Self::Lt,
            '>' => Self::Gt,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Target {
    Workflow(String),
    Accept,
    Reject,
}

impl Target {
    pub fn new(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Condition {
    field: Field,
    op: Op,
    value: i64,
    target: Target,
}

impl Condition {
    pub fn new(s: &str) -> Self {
        let (cond, target) = s.split_once(':').unwrap();
        let target = Target::new(target);

        let (field, op, value) = if cond.contains('<') {
            let (field, value) = cond.split_once('<').unwrap();
            (
                Field::new(field.chars().next().unwrap()),
                Op::Lt,
                i64::from_str_radix(value, 10).unwrap(),
            )
        } else {
            let (field, value) = cond.split_once('>').unwrap();
            (
                Field::new(field.chars().next().unwrap()),
                Op::Gt,
                i64::from_str_radix(value, 10).unwrap(),
            )
        };

        Self {
            field,
            op,
            value,
            target,
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    steps: Vec<Condition>,
    fallback: Target,
}

impl Workflow {
    pub fn new(s: &str) -> Self {
        let (name, rest) = s.split_once('{').unwrap();
        let rest = rest.replace('}', "");

        let workflows: Vec<&str> = rest.split(',').collect();

        let conditions = &workflows[0..workflows.len() - 1];
        let conditions = conditions.iter().map(|c| Condition::new(c)).collect();
        let fallback = Target::new(workflows.last().unwrap());

        Self {
            name: name.to_string(),
            steps: conditions,
            fallback,
        }
    }
}

#[derive(Debug)]
pub struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    pub fn new(s: &str) -> Self {
        let s = s.replace('{', "").replace('}', "");

        let mut fields = s.split(",");

        let x = i64::from_str_radix(fields.next().unwrap().split_once('=').unwrap().1, 10).unwrap();
        let m = i64::from_str_radix(fields.next().unwrap().split_once('=').unwrap().1, 10).unwrap();
        let a = i64::from_str_radix(fields.next().unwrap().split_once('=').unwrap().1, 10).unwrap();
        let s = i64::from_str_radix(fields.next().unwrap().split_once('=').unwrap().1, 10).unwrap();

        Self { x, m, a, s }
    }

    pub fn test(&self, cond: &Condition) -> bool {
        let value = match cond.field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        };

        match cond.op {
            Op::Lt => value < cond.value,
            Op::Gt => value > cond.value,
        }
    }

    pub fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

pub fn p1() -> std::io::Result<()> {
    let file = File::open("d19/src/input.txt")?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    for l in lines
        .by_ref()
        .take_while(|l| !l.as_ref().unwrap().is_empty())
    {
        let l = l?;
        let wf = Workflow::new(&l);
        workflows.insert(wf.name.clone(), wf);
    }

    for l in lines.by_ref() {
        let l = l?;
        parts.push(Part::new(&l));
    }

    let mut sum = 0;

    for p in parts {
        let mut workflow = Some(workflows.get("in").unwrap());

        while let Some(wf) = workflow {
            let mut matched = false;
            for c in &wf.steps {
                if p.test(&c) {
                    match &c.target {
                        Target::Workflow(s) => workflow = workflows.get(s),
                        Target::Accept => {
                            sum += p.rating();
                            workflow = None;
                        }
                        Target::Reject => {
                            workflow = None;
                        }
                    };
                    matched = true;
                    break;
                }
            }
            if !matched {
                match &wf.fallback {
                    Target::Workflow(s) => workflow = workflows.get(s),
                    Target::Accept => {
                        sum += p.rating();
                        workflow = None;
                    }
                    Target::Reject => {
                        workflow = None;
                    }
                }
            }
        }
    }

    println!("{sum}");

    Ok(())
}
