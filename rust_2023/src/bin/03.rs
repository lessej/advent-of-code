#[aoc::main(03)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

const SURROUNDS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_1(input: &str) -> usize {
    let grid = input.as_bytes();
    let mut row_len = 0;
    for c in grid {
        row_len += 1;
        if c == &b'\n' {
            break;
        }
    }

    let mut total = 0;
    let mut row: usize = 0;
    let mut col: usize = 0;

    let mut start: Option<usize> = None;
    let mut size = 0;
    let mut is_valid = false;

    for c in grid.iter() {
        match c {
            b'\n' => {
                if let Some(start) = start {
                    if is_valid {
                        let start = row * row_len + start;
                        let num: String = String::from_utf8(grid[start..(start + size)].into()).unwrap();
                        let num = num.parse::<usize>().unwrap();
                        total += num;
                    }
                }

                row += 1;
                col = 0;

                start = None;
                size = 0;
                is_valid = false;
            },
            b'0'..=b'9' => {
                if start.is_none() {
                    start = Some(col);
                }
                size += 1;

                if !is_valid {
                    for (sr, sc) in SURROUNDS.iter().copied() {
                        let Some(sr) = row.checked_add_signed(sr) else {
                            continue;
                        };

                        let Some(sc) = col.checked_add_signed(sc) else {
                            continue;
                        };

                        if sr >= grid.len() / row_len || sc >= row_len {
                            continue;
                        }

                        let idx = sr * row_len + sc;
                        if grid[idx] != b'.' && grid[idx] != b'\n' && !grid[idx].is_ascii_digit() {
                            is_valid = true;
                        }
                    }
                }
                col += 1;
            },
            _ => {
                if let Some(start) = start {
                    if is_valid {
                        let start = row * row_len + start;
                        let num: String = String::from_utf8(grid[start..(start + size)].into()).unwrap();
                        let num = num.parse::<usize>().unwrap();
                        total += num;
                    }
                }

                start = None;
                size = 0;
                is_valid = false;
                col += 1;
            },
        }
    }

    total
}

pub fn part_2(input: &str) -> usize {
    let grid = input.as_bytes();
    let mut row_len = 0;
    for c in grid {
        row_len += 1;
        if c == &b'\n' {
            break;
        }
    }

    let mut row: usize = 0;
    let mut col: usize = 0;

    let mut start: Option<usize> = None;
    let mut size = 0;
    let mut is_valid = false;
    let mut asterisk_idx: Option<usize> = None;

    let mut nums: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();

    for c in grid.iter() {
        match c {
            b'\n' => {
                if let Some(start) = start {
                    if is_valid {
                        let start = row * row_len + start;
                        let num: String = String::from_utf8(grid[start..(start + size)].into()).unwrap();
                        let num = num.parse::<usize>().unwrap();
                        
                        if let Some(operator_idx) = asterisk_idx {
                            let operands_list = nums.entry(operator_idx).or_insert(Vec::new());
                            operands_list.push(num);
                        }

                        asterisk_idx = None;
                    }
                }

                row += 1;
                col = 0;

                start = None;
                size = 0;
                is_valid = false;
            },
            b'0'..=b'9' => {
                if start.is_none() {
                    start = Some(col);
                }
                size += 1;

                if !is_valid {
                    for (sr, sc) in SURROUNDS.iter().copied() {
                        let Some(sr) = row.checked_add_signed(sr) else {
                            continue;
                        };

                        let Some(sc) = col.checked_add_signed(sc) else {
                            continue;
                        };

                        if sr >= grid.len() / row_len || sc >= row_len {
                            continue;
                        }

                        let idx = sr * row_len + sc;
                        if grid[idx] == b'*' {
                            asterisk_idx = Some(idx);
                            is_valid = true;
                        }
                    }
                }
                col += 1;
            },
            _ => {
                if let Some(start) = start {
                    if is_valid {
                        let start = row * row_len + start;
                        let num: String = String::from_utf8(grid[start..(start + size)].into()).unwrap();
                        let num = num.parse::<usize>().unwrap();

                        if let Some(operator_idx) = asterisk_idx {
                            let operands_list = nums.entry(operator_idx).or_insert(Vec::new());
                            operands_list.push(num);
                        }
                    }
                }

                start = None;
                size = 0;
                is_valid = false;
                col += 1;

                asterisk_idx = None;
            },
        }
    }

    nums
        .iter()
        .filter(|(_, list)| list.len() == 2)
        .map(|(_, list)| list.iter().product::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn d3_p1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let res = super::part_1(input);
        println!("Expected: 4361, Received: {res}");
        assert_eq!(4361, res);
    }

    #[test]
    fn d3_p2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let res = super::part_2(input);
        println!("Expected: 467835, Received: {res}");
        assert_eq!(467835, res);
    }
}
