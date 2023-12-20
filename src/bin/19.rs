#[aoc::main(19)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_val(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum PartType {
    X,
    M,
    A,
    S,
}

impl From<char> for PartType {
    fn from(value: char) -> Self {
        match value {
            'x' => PartType::X,
            'm' => PartType::M,
            'a' => PartType::A,
            's' => PartType::S,
            _ => panic!("Unexpected part type"),
        }
    }
}

#[derive(Debug)]
enum Op {
    Lt,
    Gt,
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Lt,
            '>' => Self::Gt,
            _ => panic!("Unexpected op"),
        }
    }
}

#[derive(Debug)]
struct Rule {
    part: PartType,
    op: Option<Op>,
    threshold: usize,
    next: String,
}

#[derive(Debug)]
struct Workflow {
    workflows: std::collections::HashMap<String, Vec<Rule>>,
}

impl Workflow {
    fn new() -> Self {
        Self {
            workflows: std::collections::HashMap::new(),
        }
    }

    fn parse_to_workflow(&mut self, workflows: &str) {
        for line in workflows.lines() {
            let (name, rules) = line.split_once('{').unwrap().into();
            let rules: Vec<Rule> = rules[0..(rules.len() - 1)]
                .split(',')
                .map(|rule| match rule.find(':') {
                    Some(_) => {
                        let rule: Vec<&str> = rule.split(':').collect();
                        let next = rule[1].trim().to_string();
                        let op = Op::from(rule[0].chars().nth(1).unwrap());
                        let part_type = PartType::from(rule[0].chars().nth(0).unwrap());
                        let threshold: usize = rule[0][2..].parse().unwrap();

                        Rule {
                            part: part_type,
                            op: Some(op),
                            threshold,
                            next,
                        }
                    },
                    None => Rule {
                        part: PartType::S,
                        op: None,
                        threshold: 0,
                        next: rule.to_string(),
                    }
                })
                .collect();

            self.workflows.insert(name.to_string(), rules);
        }
    }

    fn count_accepted(&self, parts: &Vec<Part>) -> usize {
        let mut total = 0;
        for part in parts {
            let mut curr_workflow = self.workflows.get("in").unwrap();
            'outer: loop {
                let mut dest = String::from("R");
                'inner: for rule in curr_workflow {
                    let part_val = match rule.part {
                        PartType::X => part.x,
                        PartType::M => part.m,
                        PartType::A => part.a,
                        PartType::S => part.s,
                    };
                    match rule.op {
                        Some(Op::Lt) => {
                            if part_val < rule.threshold {
                                dest = rule.next.clone();
                                break 'inner;
                            }
                        },
                        Some(Op::Gt) => {
                            if part_val > rule.threshold {
                                dest = rule.next.clone();
                                break 'inner;
                            }

                        },
                        None => dest = rule.next.clone(),
                    }
                }
                match dest.chars().next().unwrap() {
                    'A' => {
                        total += part.get_val();
                        break 'outer;
                    },
                    'R' => break 'outer,
                    _ => curr_workflow = self.workflows.get(&dest).unwrap(),
                }
            }
        }

        total
    }
}

pub fn part_1(input: &str) -> usize {
    let (workflows, parts) : (&str, &str) = input
        .split_once("\n\n")
        .unwrap()
        .into();

    let parts: Vec<Part> = parts
        .lines()
        .map(|line| { 
            let line = line[1..(line.len() - 1)].to_string();
            let vals: Vec<&str> = line.split(',').collect();
            let x = vals[0][2..].parse::<usize>().unwrap();
            let m = vals[1][2..].parse::<usize>().unwrap();
            let a = vals[2][2..].parse::<usize>().unwrap();
            let s = vals[3][2..].parse::<usize>().unwrap();

            Part {
                x,
                m,
                a,
                s,
            }
        })
        .collect();

    let mut workflow = Workflow::new();
    workflow.parse_to_workflow(&workflows);
    let total = workflow.count_accepted(&parts);

    total
}

pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn d19_p1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        let res = super::part_1(input);
        println!("Expected: 19114, Received: {res}");
        assert_eq!(19114, res);
    }

    #[test]
    fn d19_p2() {
        let input = "";

        let res = super::part_2(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }
}
