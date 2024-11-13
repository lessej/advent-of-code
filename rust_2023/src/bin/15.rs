#[aoc::main(15)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let steps = input
        .trim_end()
        .split(',')
        .collect::<Vec<&str>>();

    steps
        .iter()
        .map(|step| {
            step
                .as_bytes()
                .iter()
                .fold(0, |acc, c| {
                    let mut curr = acc.to_owned();
                    curr += usize::from(*c);
                    curr *= 17;
                    curr % 256
                })
        })
        .fold(0, |acc, step| acc + step)
}

fn hash_val(input: &str) -> usize {
    input.as_bytes()
        .iter()
        .fold(0, |acc, c| {
            let mut curr = acc.to_owned();
            curr += usize::from(*c);
            curr *= 17;
            curr % 256
        })
}

enum Op {
    Assign,
    Remove
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '=' => Op::Assign,
            '-' => Op::Remove,
            _ => panic!("Unexpected operator"),
        }
    }
}

pub fn part_2(input: &str) -> usize {
    let steps = input
        .trim_end()
        .split(',')
        .collect::<Vec<&str>>();

    let mut boxes: Vec<Vec<(String, usize)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for step in steps {
        let split_idx = step.find(|c| c == '=' || c == '-').unwrap();
        let label = step[0..split_idx].to_string();
        let op = step.chars().collect::<Vec<char>>()[split_idx];
        let hash_key = hash_val(label.as_str());

        match Op::from(op) {
            Op::Assign => {
                let focal_len = step[(split_idx + 1)..].parse::<usize>().unwrap();
                if let Some((_, v)) = boxes[hash_key].iter_mut().find(|(k, _)| *k == label) {
                    *v = focal_len;
                } else {
                    boxes[hash_key].push((label, focal_len));
                }
            },
            Op::Remove => {
                if let Some(idx) = boxes[hash_key].iter_mut().position(|(k, _)| *k == label) {
                    boxes[hash_key].remove(idx);
                }
            },
        }
    }

    let mut total_focusing_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, fl)) in b.iter().enumerate() {
            total_focusing_power += (i + 1) * (j + 1) * fl;
        }
    }

    total_focusing_power
}

#[cfg(test)]
mod test {
    #[test]
    fn d15_p1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

        let res = super::part_1(input);
        println!("Expected: 1320, Received: {res}");
        assert_eq!(1320, res);
    }

    #[test]
    fn d15_p2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

        let res = super::part_2(input);
        println!("Expected: 145, Received: {res}");
        assert_eq!(145, res);
    }
}
