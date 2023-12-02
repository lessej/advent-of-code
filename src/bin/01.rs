#[aoc::main(01)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);
    
    (p1,p2)
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    let mut start: Option<char> = None;
    let mut end: Option<char> = None;
    let input = format!("{input}\n");

    for c in input.chars() {
        match c {
            '0'..='9' => {
                if start.is_none() {
                    start = Some(c);
                    end = Some(c);
                } else {
                    end = Some(c);
                }
            },
            '\n' => {
                if let (Some(start), Some(end)) = (start, end) {
                    let line_total = format!("{start}{end}").parse::<usize>().unwrap();
                    total += line_total;
                }

                start = None;
                end = None;
            },
            _ => {}
        }
    }

    total
}

fn part_2(input: &str) -> usize {
    let mut total = 0;
    let mut start: Option<u8> = None;
    let mut end: Option<u8> = None;
    let input = format!("{input}\n");
    let input: Vec<u8> = input.as_bytes().into();

    let nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let num_map = std::collections::HashMap::from([
        ("one", b'1'),
        ("two", b'2'),
        ("three", b'3'),
        ("four", b'4'),
        ("five", b'5'),
        ("six", b'6'),
        ("seven", b'7'),
        ("eight", b'8'),
        ("nine", b'9'),
    ]);

    fn matches(target: &str, input: &Vec<u8>, curr_idx: usize, num_map: &std::collections::HashMap<&str, u8>) -> Option<u8> {
        let mut curr_len = 1;
        let mut i = curr_idx;
        let target_bytes = target.as_bytes();
        while curr_len <= target.len() && target_bytes[i - curr_idx] == input[i] {
            curr_len += 1;
            i += 1;
        }

        if curr_len - 1 != target.len() {
            return None;
        }

        Some(num_map.get(target).unwrap().clone())
    }

    for (idx, c) in input.iter().enumerate() {
        match c {
            b'o' | b't' | b'f' | b's' | b'e' | b'n' => {
                for tar in &nums {
                    if let Some(num) = matches(tar, &input, idx, &num_map) {
                        if start.is_none() {
                            start = Some(num);
                        }
                        end = Some(num);
                    }
                }
            }
            b'0'..=b'9' => {
                if start.is_none() {
                    start = Some(c.clone());
                }
                end = Some(c.clone());
            },
            b'\n' => {
                if let (Some(start), Some(end)) = (start, end) {
                    let start = vec![start];
                    let end = vec![end];
                    let start = std::str::from_utf8(&start).unwrap();
                    let end = std::str::from_utf8(&end).unwrap();
                    let line_total = format!("{start}{end}").parse::<usize>().unwrap();
                    total += line_total;
                }

                start = None;
                end = None;
            },
            _ => {}
        }
    }

    total
}

#[cfg(test)]
mod test {
    #[test]
    fn d1_p1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let res = super::part_1(input);
        println!("Expected: 142, Given: {res}");
        assert_eq!(142, res);
    }

    #[test]
    fn d1_p2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let res = super::part_2(input);
        println!("Expected: 281, Given: {res}");
        assert_eq!(res, 281);
    }
}
