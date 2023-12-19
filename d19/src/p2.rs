use std::{
    collections::HashMap,
    fmt::Display,
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
    original: String,
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
            original: s.to_string(),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> i64 {
        self.end - self.start
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if end < start {
            None
        } else {
            Some(Range { start, end })
        }
    }

    // Returns two ranges, one fulfilling and one failing
    pub fn split(&self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        let (pass, fail) = match cond.op {
            Op::Lt => (Range::new(0, cond.value), Range::new(cond.value, 4001)),
            Op::Gt => (
                Range::new(cond.value + 1, 4001),
                Range::new(0, cond.value + 1),
            ),
        };

        (pass.intersect(self), fail.intersect(self))
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:04}", self.start, self.end - 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PPart {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl Display for PPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, m: {}, a: {}, s: {}",
            self.x, self.m, self.a, self.s
        )
    }
}

impl PPart {
    pub fn new() -> Self {
        Self {
            x: Range::new(1, 4001),
            m: Range::new(1, 4001),
            a: Range::new(1, 4001),
            s: Range::new(1, 4001),
        }
    }

    pub fn set(&self, f: &Field, r: &Range) -> Self {
        match f {
            Field::X => Self { x: *r, ..*self },
            Field::M => Self { m: *r, ..*self },
            Field::A => Self { a: *r, ..*self },
            Field::S => Self { s: *r, ..*self },
        }
    }

    pub fn get(&self, field: &Field) -> Range {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    pub fn size(&self) -> i64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    pub fn apply(&self, cond: &Condition) -> (Option<Self>, Option<Self>) {
        let range = self.get(&cond.field);

        let (rpass, rfail) = range.split(&cond);

        let pass = rpass.map(|p| self.set(&cond.field, &p));
        let fail = rfail.map(|p| self.set(&cond.field, &p));

        (pass, fail)
    }

    pub fn valid(&self, workflow: &Workflow, wfs: &HashMap<String, Workflow>) -> Vec<PPart> {
        println!("{}: {}", workflow.name, self);
        println!("---------");
        let mut pparts: Vec<PPart> = Vec::new();

        let mut range = *self;

        for c in &workflow.steps {
            println!("STEP: {}", c);
            let (pass, fail) = range.apply(&c);
            println!("PASS: {}", pass.unwrap());
            println!("FAIL: {}", fail.unwrap());
            match pass {
                Some(pass) => match &c.target {
                    Target::Workflow(s) => {
                        println!("RECURSE: {}", pass);
                        let next = wfs.get(s).unwrap();
                        pparts.extend(pass.valid(&next, wfs));
                    }
                    Target::Accept => {
                        println!("ACCEPT: {}", pass);
                        pparts.push(pass);
                    }
                    Target::Reject => {}
                },
                None => {}
            }

            match fail {
                Some(fail) => {
                    println!("FAIL: {}", fail);
                    range = fail
                }
                None => {}
            }
        }

        match &workflow.fallback {
            Target::Workflow(s) => {
                let next = wfs.get(s).unwrap();
                pparts.extend(range.valid(&next, wfs));
            }
            Target::Accept => {
                pparts.push(range);
            }
            Target::Reject => {}
        }

        println!("RESULT FOR {}:", workflow.name);
        for p in &pparts {
            println!("{}", p);
        }
        println!("-------");

        pparts
    }
}

pub fn p2() -> std::io::Result<()> {
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

    let result = PPart::new().valid(workflows.get("in").unwrap(), &workflows);

    let size: i64 = result.iter().map(|r| r.size()).sum();
    println!("{size}");

    Ok(())
}
