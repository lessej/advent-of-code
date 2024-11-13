#[aoc::main(09)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

fn part_1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
        )
        .collect::<Vec<Vec<i64>>>();

    let mut total = 0;
    for line in lines {
        let mut last_map: Vec<i64> = Vec::new();
        let mut curr_line = line.clone();
        let mut has_non_zero = true;
        while has_non_zero {
            let mut line_diff: Vec<i64> = Vec::new();
            let mut needs_continue = false;
            for (back, front) in curr_line.iter().zip(curr_line.iter().skip(1)) {
                let diff = front - back;
                if diff != 0 {
                    needs_continue = true;
                }
                line_diff.push(diff);
            }
            if needs_continue {
                has_non_zero = true;
                curr_line = line_diff;
                last_map.push(*curr_line.last().unwrap());
            } else {
                has_non_zero = false;
            }
        }

        last_map.push(line.last().unwrap().to_owned());
        total += last_map
            .iter()
            .rev()
            .fold(0, |acc, curr| acc + curr);
    }

    total as usize
}

fn part_2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
        )
        .collect::<Vec<Vec<i64>>>();

    let mut total = 0;
    for line in lines {
        let mut first_map: Vec<i64> = Vec::new();
        let mut curr_line = line.clone();
        let mut has_non_zero = true;
        while has_non_zero {
            let mut line_diff: Vec<i64> = Vec::new();
            has_non_zero = false;
            for (back, front) in curr_line.iter().zip(curr_line.iter().skip(1)) {
                let diff = front - back;
                if diff != 0 {
                    has_non_zero = true;
                }
                line_diff.push(diff);
            }
            if has_non_zero {
                curr_line = line_diff;
                first_map.push(*curr_line.first().unwrap());
            }
        }
        
        first_map.insert(0, line.first().unwrap().to_owned());

        let mut prev_val = 0;
        for leftmost in first_map.iter().rev() {
            prev_val = leftmost - prev_val;
        }

        total += prev_val;
    }

    total as usize
}

#[cfg(test)]
mod test {
    #[test]
    fn d9_p1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let res = super::part_1(input);
        println!("Expected: 114, Received: {res}");
        assert_eq!(114, res);
    }

    #[test]
    fn d9_p2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let res = super::part_2(input);
        println!("Expected: 2, Received: {res}");
        assert_eq!(2, res);
    }
}
