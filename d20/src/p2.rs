use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    process::Output,
};

#[derive(Debug, Clone)]
pub struct FFState {
    state: bool,
}

impl FFState {
    pub fn new() -> Self {
        Self { state: false }
    }
}

#[derive(Debug, Clone)]
pub struct ConjState {
    inputs: HashMap<String, bool>,
}

impl ConjState {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleType {
    Broadcast,
    FlipFlop(FFState),
    Conjunction(ConjState),
    Output,
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    ty: ModuleType,
}

impl Module {
    pub fn new(s: &str) -> Self {
        let first = s.chars().next().unwrap();

        match first {
            '%' => Self {
                name: s.split_at(1).1.to_string(),
                ty: ModuleType::FlipFlop(FFState::new()),
            },
            '&' => Self {
                name: s.split_at(1).1.to_string(),
                ty: ModuleType::Conjunction(ConjState::new()),
            },
            _ => {
                if s == "broadcaster" {
                    Self {
                        name: s.to_string(),
                        ty: ModuleType::Broadcast,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    from: String,
    target: String,
    pulse: bool,
}

pub struct Circuit {
    modules: HashMap<String, Module>,
    outputs: HashMap<String, Vec<String>>,
    inputs: HashMap<String, Vec<String>>,

    high: i64,
    low: i64,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            outputs: HashMap::new(),
            inputs: HashMap::new(),
            high: 0,
            low: 0,
        }
    }

    pub fn add_module(&mut self, l: &str) {
        let (name, targets) = l.split_once("->").unwrap();

        let name = name.trim();
        let targets = targets.trim();

        let module = Module::new(name);
        self.modules.insert(module.name.clone(), module.clone());

        let targets = targets.split(", ");

        for t in targets {
            let (from, to) = (&module.name, t);
            if !self.outputs.contains_key(from) {
                self.outputs.insert(from.to_string(), Vec::new());
            }
            if !self.inputs.contains_key(to) {
                self.inputs.insert(to.to_string(), Vec::new());
            }

            self.outputs.get_mut(from).unwrap().push(to.to_string());
            self.inputs.get_mut(to).unwrap().push(from.to_string());
        }
    }

    pub fn post_init(&mut self) {
        for (name, v) in self.modules.iter_mut() {
            if let ModuleType::Conjunction(ref mut state) = &mut v.ty {
                let inputs = self.inputs.get(name).unwrap();
                for i in inputs {
                    state.inputs.insert(i.clone(), false);
                }
            }

            if !self.outputs.contains_key(name) {
                self.outputs.insert(name.clone(), Vec::new());
            }

            if !self.inputs.contains_key(name) {
                self.inputs.insert(name.clone(), Vec::new());
            }
        }

        self.modules.insert(
            "output".to_string(),
            Module {
                name: "output".to_string(),
                ty: ModuleType::Output,
            },
        );
    }

    pub fn propagate_one(&mut self, event: &Event, iter: usize) -> Vec<Event> {
        // println!("PROPAGATING:");
        // println!("{} -> {}: {}", event.from, event.target, event.pulse);
        if event.pulse {
            self.high += 1;
        } else {
            self.low += 1;
        }
        let mut events = Vec::new();

        if event.from == "st" || event.from == "hh" || event.from == "tn" || event.from == "dt" {
            if event.pulse {
                println!("#{}: {} {}", iter, event.from, event.pulse);
            }
        }

        if !self.modules.contains_key(&event.target) {
            return Vec::new();
        }

        let target = self.modules.get_mut(&event.target).unwrap();
        match &mut target.ty {
            ModuleType::Broadcast => {
                for o in self.outputs.get(&target.name).unwrap() {
                    events.push(Event {
                        from: target.name.to_string(),
                        target: o.to_string(),
                        pulse: event.pulse,
                    })
                }
            }
            ModuleType::FlipFlop(ref mut state) => {
                if !event.pulse {
                    state.state = !state.state;
                    for o in self.outputs.get(&target.name).unwrap() {
                        events.push(Event {
                            from: target.name.to_string(),
                            target: o.to_string(),
                            pulse: state.state,
                        })
                    }
                }
            }
            ModuleType::Conjunction(ref mut state) => {
                state.inputs.insert(event.from.clone(), event.pulse);

                let output = !state.inputs.values().all(|b| *b == true);

                for o in self.outputs.get(&target.name).unwrap() {
                    events.push(Event {
                        from: target.name.to_string(),
                        target: o.to_string(),
                        pulse: output,
                    })
                }
            }
            ModuleType::Output => {}
        }

        events
    }
}

pub fn p2() -> std::io::Result<()> {
    let file = File::open("d20/src/input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut circuit = Circuit::new();

    for l in buf_reader.lines() {
        let l = l?;
        circuit.add_module(&l);
    }

    circuit.post_init();

    for i in 0..1000000000 {
        let mut queue: VecDeque<Event> = VecDeque::new();
        queue.push_back(Event {
            from: "button".to_string(),
            target: "broadcaster".to_string(),
            pulse: false,
        });

        while let Some(event) = queue.pop_front() {
            let result = circuit.propagate_one(&event, i);
            for r in result {
                queue.push_back(r);
            }
        }
    }

    println!("{}", circuit.low);
    println!("{}", circuit.high);
    println!("{}", circuit.low * circuit.high);

    Ok(())
}
