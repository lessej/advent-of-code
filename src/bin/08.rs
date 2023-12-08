#[aoc::main(08)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();

    let instructions = blocks[0].as_bytes();
    let mut stages: std::collections::HashMap<String, (String, String)> = std::collections::HashMap::new();
    blocks[1]
        .lines()
        .for_each(|line| {
            let key = line[..=2].to_string();
            let left = line[7..=9].to_string();
            let right = line[12..=14].to_string();

            stages.entry(key).or_insert((left, right));
        });

    let mut steps = 0;
    let mut curr_key = "AAA";
    while curr_key != "ZZZ" {
        let idx = steps % &instructions.len();
        curr_key = match instructions[idx] {
            b'L' => stages.get(curr_key).unwrap().0.as_str(),
            b'R' => stages.get(curr_key).unwrap().1.as_str(),
            _ => curr_key
        };
        steps += 1;

    }

    steps
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

fn lcm(a: &usize, b: &usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn part_2(input: &str) -> usize {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();

    let instructions = blocks[0].as_bytes();
    let mut stages: std::collections::HashMap<String, (String, String)> = std::collections::HashMap::new();
    let mut start_keys: Vec<String> = Vec::new();
    blocks[1]
        .lines()
        .for_each(|line| {
            let key = line[..=2].to_string();
            let left = line[7..=9].to_string();
            let right = line[12..=14].to_string();

            if line.chars().nth(2).unwrap() == 'A' {
                start_keys.push(key.clone());
            }

            stages.entry(key.clone()).or_insert((left, right));
        });

    start_keys
        .iter()
        .map(|start_key| {
            let mut steps = 0;
            let mut curr_key = start_key.as_str();
            while curr_key.chars().nth(2).unwrap() != 'Z' {
                let idx = steps % &instructions.len();
                curr_key = match instructions[idx] {
                    b'L' => stages.get(curr_key).unwrap().0.as_str(),
                    b'R' => stages.get(curr_key).unwrap().1.as_str(),
                    _ => curr_key
                };
                steps += 1;
            }
            steps
        })
        .collect::<Vec<usize>>()
        .iter()
        .fold(1, |curr_lcm, curr_key| { 
            lcm(&curr_lcm, &curr_key) 
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn d8_p1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let res = super::part_1(input);
        println!("Expected: 2, Received: {res}");
        assert_eq!(2, res);
    }

    #[test]
    fn d8_p2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let res = super::part_2(input);
        println!("Expected: 6, Received: {res}");
        assert_eq!(6, res);
    }
}
