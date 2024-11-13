#[aoc::main(22)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    0
}

pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn d21_p1() {
        let input = "";

        let res = super::part_1(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }

    #[test]
    fn d21_p2() {
        let input = "";

        let res = super::part_2(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }
}
