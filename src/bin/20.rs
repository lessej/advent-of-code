use std::collections::HashMap;

#[aoc::main(20)]
pub fn main(input: &str) -> (usize, usize) {
    // let p1 = part_1(input);
    let p1 = 0;
    let p2 = part_2(input);

    (p1, p2)
}

enum Module {
    FlipFlop(bool),
    Conjunction(std::collections::HashMap<String, bool>),
    Broadcaster,
}

impl Module {
    fn init_conjunc(&mut self, inputs: Vec<String>) {
        if let Module::Conjunction(input_levels) = self {
            input_levels.extend(inputs.into_iter().map(|i| (i, false)));
        }
    }

    fn sig_in_out(&mut self, signal: Signal) -> (Option<bool>, Option<String>) {
        match self {
            Module::FlipFlop(state) => {
                if signal.level {
                    (None, None)
                } else {
                    *state = !*state;
                    (Some(*state), None)
                }
            },
            Module::Conjunction(input_levels) => {
                *input_levels.get_mut(&signal.from).unwrap() = signal.level;
                let is_all_high = input_levels.values().all(|l| *l);
                if is_all_high {
                    (Some(!is_all_high), Some(signal.from))
                } else {
                    (Some(!is_all_high), None)
                }
            },
            Module::Broadcaster => (Some(signal.level), None),
        }
    }
}

struct Signal {
    from: String,
    to: String,
    level: bool,
}

impl Signal {
    fn new(from: String, to: String, level: bool) -> Self {
        Self {
            from,
            to,
            level,
        }
    }
}

struct Modules {
    modules: std::collections::HashMap<String, Module>,
    outputs: std::collections::HashMap<String, Vec<String>>,
    conjs: std::collections::HashMap<String, Option<usize>>,
    low_pulse_count: usize,
    high_pulse_count: usize,
    button_count: usize,
}

impl Modules {
    fn check_all_set(&self) -> bool {
        println!("checks: {:?}", self.conjs);
        self.conjs.values().into_iter().all(|c| c.is_some())
    }

    fn push_button(&mut self) -> Option<bool> {
        self.button_count += 1;
        let mut queue = std::collections::VecDeque::new();
        let start_sig = Signal::new("button".to_string(), "broadcaster".to_string(), false);
        queue.push_back(start_sig);

        while let Some(curr_sig) = queue.pop_front() {
            match curr_sig.level {
                true => self.high_pulse_count += 1,
                false => self.low_pulse_count += 1,
            }

            let target = &(curr_sig.to.clone());
            let from = &(curr_sig.from.clone());
            if let Some(module) = self.modules.get_mut(target) {
                if let (Some(new_level), is_finished) = module.sig_in_out(curr_sig) {
                    if let Some(fin_mod) = is_finished {
                        if CONJUNCTIONS.contains(&fin_mod.as_str()) {
                            if let Some(conj) = self.conjs.get_mut(&fin_mod) {
                                *conj = Some(self.button_count);
                                if self.check_all_set() {
                                    return Some(true);
                                }
                            }
                        }
                    }
                    let outputs = self.outputs.get(target).unwrap();
                    for output in outputs {
                        let new_sig = Signal::new(target.clone(), output.clone(), new_level);
                        queue.push_back(new_sig);
                    }
                }
            }
        }

        None
    }
}

pub fn part_1(input: &str) -> usize {
    let mut modules: std::collections::HashMap<String, Module> = std::collections::HashMap::new();
    let mut inputs: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut outputs: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for line in input.lines() {
        let (mod_name, targets) = line.split_once(" -> ").unwrap().into();
        let targets: Vec<String> = targets
            .split(',')
            .map(|t| t.trim().to_string())
            .collect();
        let (module, name) = match (&mod_name[0..1], &mod_name[1..]) {
            ("%", name) => (Module::FlipFlop(false), name),
            ("&", name) => (Module::Conjunction(std::collections::HashMap::new()), name),
            ("b", "roadcaster") => (Module::Broadcaster, mod_name),
            _ => panic!("Unexpected module name"),
        };

        outputs.insert(name.to_string(), targets.clone());
        for target in targets {
            inputs.entry(target).or_default().push(name.to_string());
        }
        modules.insert(name.to_string(), module);
    }

    for (mod_name, input_names) in inputs {
        if let Some(module) = modules.get_mut(&mod_name) {
            module.init_conjunc(input_names);
        }
    }

    let mut modules = Modules {
        modules,
        outputs,
        conjs: std::collections::HashMap::new(),
        low_pulse_count: 0,
        high_pulse_count: 0,
        button_count: 0,
    };

    for _ in 0..1000 {
        modules.push_button();
    }

    modules.low_pulse_count * modules.high_pulse_count
}

const CONJUNCTIONS: &[&str; 4] = &["fh", "mf", "fz", "ss"];

pub fn part_2(input: &str) -> usize {
    let mut modules: std::collections::HashMap<String, Module> = std::collections::HashMap::new();
    let mut inputs: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut outputs: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut conjs: std::collections::HashMap<String, Option<usize>> = std::collections::HashMap::new();

    for line in input.lines() {
        let (mod_name, targets) = line.split_once(" -> ").unwrap().into();
        let targets: Vec<String> = targets
            .split(',')
            .map(|t| t.trim().to_string())
            .collect();
        let (module, name) = match (&mod_name[0..1], &mod_name[1..]) {
            ("%", name) => (Module::FlipFlop(false), name),
            ("&", name) => {
                if CONJUNCTIONS.contains(&name) {
                    conjs.insert(name.to_string(), None);
                }
                (Module::Conjunction(std::collections::HashMap::new()), name)
            },
            ("b", "roadcaster") => (Module::Broadcaster, mod_name),
            _ => panic!("Unexpected module name"),
        };

        outputs.insert(name.to_string(), targets.clone());
        for target in targets {
            inputs.entry(target).or_default().push(name.to_string());
        }
        modules.insert(name.to_string(), module);
    }

    println!("checks: {:?}", conjs);

    for (mod_name, input_names) in inputs {
        if let Some(module) = modules.get_mut(&mod_name) {
            module.init_conjunc(input_names);
        }
    }

    let mut modules = Modules {
        modules,
        outputs,
        conjs,
        low_pulse_count: 0,
        high_pulse_count: 0,
        button_count: 0,
    };

    loop {
        let res = modules.push_button();
        if res.is_some() {
            break;
        }
    }

    let cycle_lens: Vec<usize> = modules.conjs.values().map(|cl| cl.unwrap()).collect();
    println!("{:?}", cycle_lens);

    lcm(&cycle_lens)
}

fn gcd(a: &usize, b: &usize) -> usize {
    let mut larger = a.clone();
    let mut smaller = b.clone();
    if a < b {
        larger = *b;
        smaller = *a;
    }

    loop {
        let rem = larger % smaller;
        if rem == 0 {
            return smaller;
        }

        larger = smaller;
        smaller = rem;
    }
}

fn lcm(arr: &Vec<usize>) -> usize {
    let b = arr.len();
    let mut a = arr[0];
    for i in 1..b {
        a = ((arr[i] * a)) / (gcd(&arr[i], &a));

    }

    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn d20_p1() {
        let input_1 = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let input_2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";


        // let res_1 = super::part_1(input_1);
        let res_2 = super::part_1(input_2);

        // println!("Expected: 32000000, Received: {res_1}");
        // assert_eq!(32000000, res_1);

        println!("Expected: 11687500, Received: {res_2}");
        assert_eq!(11687500, res_2);
    }

    #[test]
    fn d20_p2() {
        let input = "";

        let res = super::part_2(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }
}
