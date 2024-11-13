#[aoc::main(06)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| 
            line
                .split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        )
        .collect::<Vec<Vec<usize>>>();

    lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(time, dist)| {
            let mut total = 0;
            for t in 0..=*time {
                if t * (*time - t) > *dist {
                    total += 1;
                }
            }
            total
        })
        .fold(1, |acc, x| acc * x)
}

pub fn part_2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| 
            line
                .split(':')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .collect::<String>()
                .parse::<usize>().unwrap()
        )
        .collect::<Vec<usize>>();

    let mut start = 0;
    let mut end = 0;

    for t in 0..=lines[0] {
        if t * (lines[0] - t) > lines[1] {
            start = t;
            break;
        }
    }
    for t in (0..=lines[0]).rev() {
        if t * (lines[0] - t) > lines[1] {
            end = t;
            break;
        }
    }

    end - start + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn d6_p1() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";

        let res = super::part_1(input);
        println!("Expected: 288, Received: {res}");
        assert_eq!(288, res);
    }

    #[test]
    fn d6_p2() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";

        let res = super::part_2(input);
        println!("Expected: 71503, Received: {res}");
        assert_eq!(71503, res);
    }
}
