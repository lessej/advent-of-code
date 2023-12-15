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
pub fn part_2(input: &str) -> usize {
    0
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let res = super::part_2(input);
        println!("Expected: 1320, Received: {res}");
        assert_eq!(1320, res);
    }
}
