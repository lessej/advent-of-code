#[aoc::main(10)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

const SURROUNDS: &[(isize, isize)] = &[
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 0),
];

fn get_next(curr_idx: (usize, usize), prev_idx: (usize, usize), grid: &Vec<Vec<char>>) -> (usize, usize) {
    let curr_symbol = grid[curr_idx.0][curr_idx.1];
    match curr_symbol {
        '-' => {
            if prev_idx.1 < curr_idx.1 {
                return (curr_idx.0, curr_idx.1 + 1);
            } else {
                return (curr_idx.0, curr_idx.1 - 1);
            }
        },
        '|' => {
            if prev_idx.0 < curr_idx.0 {
                return (curr_idx.0 + 1, curr_idx.1);
            } else {
                return (curr_idx.0 - 1, curr_idx.1);
            }
        },
        'J' => {
            if prev_idx.0 < curr_idx.0 {
                return (curr_idx.0, curr_idx.1 - 1);
            } else {
                return (curr_idx.0 - 1, curr_idx.1);
            }
        },
        'L' => {
            if prev_idx.0 < curr_idx.0 {
                return (curr_idx.0, curr_idx.1 + 1);
            } else {
                return (curr_idx.0 - 1, curr_idx.1);
            }
        },
        '7' => {
            if prev_idx.0 > curr_idx.0 {
                return (curr_idx.0, curr_idx.1 - 1);
            } else {
                return (curr_idx.0 + 1, curr_idx.1);
            }
        },
        'F' => {
            if prev_idx.0 > curr_idx.0 {
                return (curr_idx.0, curr_idx.1 + 1);
            } else {
                return (curr_idx.0 + 1, curr_idx.1);
            }
        }
        _ => return (0, 0),
    }
}

fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut start: (usize, usize) = (0, 0);
    let mut next: (usize, usize) = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                for (i, (sr, sc)) in SURROUNDS.iter().copied().enumerate() {
                    let (Some(sr), Some(sc)) = (row.checked_add_signed(sr), col.checked_add_signed(sc)) else {
                        continue;
                    };

                    if sr >= grid.len() || sc >= grid[0].len() {
                        continue;
                    }

                    match i {
                        0 => {
                            match grid[sr][sc] {
                                '|' | 'F' | '7' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        1 => {
                            match grid[sr][sc] {
                                'L' | '-' | 'F' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        2 => {
                            match grid[sr][sc] {
                                '-' | 'J' | '7' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        3 => {
                            match grid[sr][sc] {
                                '|' | 'J' | 'L' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }

                }
                start = (row, col);
                break;
            }
        }
    }

    // println!("Start: ({},{}), next: ({},{})", start.0, start.1, next.0, next.1);


    let mut prev: (usize, usize) = start.clone();
    let mut is_loop_complete = false;
    let mut loop_size = 1;
    while !is_loop_complete {
        let temp_prev = next.clone();
        next = get_next(next, prev, &grid);
        prev = temp_prev;
        // println!("next: {}, {}", next.0, next.1);
        if next == start {
            is_loop_complete = true;
        }
        loop_size += 1;
    }


    loop_size / 2
}

fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut start: (usize, usize) = (0, 0);
    let mut next: (usize, usize) = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                for (i, (sr, sc)) in SURROUNDS.iter().copied().enumerate() {
                    let (Some(sr), Some(sc)) = (row.checked_add_signed(sr), col.checked_add_signed(sc)) else {
                        continue;
                    };

                    if sr >= grid.len() || sc >= grid[0].len() {
                        continue;
                    }

                    match i {
                        0 => {
                            match grid[sr][sc] {
                                '|' | 'F' | '7' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        1 => {
                            match grid[sr][sc] {
                                'L' | '-' | 'F' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        2 => {
                            match grid[sr][sc] {
                                '-' | 'J' | '7' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        3 => {
                            match grid[sr][sc] {
                                '|' | 'J' | 'L' => {
                                    next = (sr, sc);
                                    break;
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }

                }
                start = (row, col);
                break;
            }
        }
    }

    let mut loop_chars: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut prev: (usize, usize) = start.clone();
    loop_chars.insert(start);
    while loop_chars.get(&next).is_none() {
        loop_chars.insert(next);
        let temp_prev = next.clone();
        next = get_next(next, prev, &grid);
        prev = temp_prev;
    }

    let mut inside: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    for row in 1..grid.len() {
        for col in 1..grid[0].len() {
            if loop_chars.get(&(row, col)).is_none() {
                let mut wall_count = 0;
                let mut i = col;
                let mut is_open_f = false;
                let mut is_open_l = false;
                while i < grid[0].len() {
                    if loop_chars.get(&(row, i)).is_some() {
                        match grid[row][i] {
                            'F' => is_open_f = true,
                            'L' => is_open_l = true,
                            'J' => {
                                if is_open_f {
                                    wall_count += 1;
                                    is_open_f = false;
                                }
                                is_open_l = false;
                            },
                            '7' => {
                                if is_open_l {
                                    wall_count += 1;
                                    is_open_l = false;
                                }
                                is_open_f = false;
                            }
                            '|' => wall_count += 1,
                            _ => {}

                        }
                    } else {
                        is_open_f = false;
                        is_open_l = false;
                    }
                    i += 1;

                }

                if wall_count % 2 != 0 {
                    inside.insert((row, col));
                }
            }
        }
    }

    let count = inside.len();

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn d10_p1() {
        let input_simple = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let input_hard = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";


        let res_simple = super::part_1(input_simple);
        let res_hard = super::part_1(input_hard);

        println!("Simple | Expected: 4, Received: {res_simple}");
        assert_eq!(4, res_simple);

        println!("Hard | Expected: 8, Received: {res_hard}");
        assert_eq!(8, res_hard);
    }

    #[test]
    fn d10_p2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";


        let res = super::part_2(input);
        println!("Expected: 10, Received: {res}");
        assert_eq!(10, res);
    }
}
