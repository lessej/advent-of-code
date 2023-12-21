use std::{hash::{Hasher, Hash}, borrow::Borrow};

#[aoc::main(20)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn flip(&self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

struct Module {
    op: Op,
    targets: Vec<String>,
    hist: Vec<Pulse>,

}

struct Modules {
    modules: std::collections::HashMap<String, Module>,
}

impl Modules {
    fn get_next(&self, module: String, pulse: Pulse) -> (Vec<String>, Pulse) {
        let curr_mod = self.modules.get_mut(&module).unwrap();
        match curr_mod.op {
            Op::FlipFlop => {
                match pulse {
                    Pulse::High => (Vec::new(), pulse),
                    Pulse::Low => (curr_mod.targets, pulse.flip())
                }
            },
            Op::Broadcaster => (curr_mod.targets, pulse),
            Op::Conjunction => {
                curr_mod.hist.push(pulse);
                match curr_mod.hist.iter().filter(|p| *p == &Pulse::Low).count() {
                    0 => (curr_mod.targets, Pulse::Low),
                    _ => (curr_mod.targets, Pulse::High),
                }

            },
        }
    }
}



pub fn part_1(input: &str) -> usize {
    let mut modules = std::collections::HashMap::new();
    let mut rules = Vec::new();
    rules.push(Rule { name: "button".to_string(), targets: vec!["broadcaster".to_string()] });
    modules.insert("button".to_string(), Module { op: Op::Broadcast(Pulse::Low), targets: vec!["broadcaster".to_string()]});
    for line in input.lines() {
        let (mod_name, targets) = line.split_once("->").unwrap().into();
        let (mod_name, op) = match mod_name.chars().next().unwrap() {
            'b' => ("broadcaster", Op::Broadcast(Pulse::Low)),
            _ => {
                let name_op = mod_name.trim();
                (&name_op[1..], Op::from(mod_name.chars().next().unwrap()))
            }
        };
        let targets: Vec<String> = targets
            .split(',')
            .map(|t| t.trim().to_string())
            .collect();

        rules.push(Rule {
            name: mod_name.to_string(),
            targets,
        });
        modules.insert(mod_name.to_string(), Module {
            op,
            targets,
        });
    }
    0
}

pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn d20_p1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let res = super::part_1(input);
        println!("Expected: 32000000, Received: {res}");
        assert_eq!(32000000, res);
    }

    #[test]
    fn d20_p2() {
        let input = "";

        let res = super::part_2(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }
}
