#[aoc::main(14)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    tilt_north(&mut grid);
    check_load(&grid)
}

fn check_load(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match grid[row][col] {
                'O' => total += grid.len() - row,
                _ => {}
            }
        }
    }

    total
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut avail = 0;

        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    if avail != row {
                        grid[avail][col] = 'O';
                        grid[row][col] = '.';
                    }
                    avail += 1;
                },
                '#' => {
                    avail = row + 1;
                },
                _ => {},
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut avail = grid.len() - 1;

        for row in (0..grid.len()).rev() {
            match grid[row][col] {
                'O' => {
                    if avail != row {
                        grid[avail][col] = 'O';
                        grid[row][col] = '.';
                    }
                    if let Some(check_avail) = avail.checked_add_signed(-1) {
                        avail = check_avail;
                    }
                },
                '#' => {
                    if let Some(check_avail) = row.checked_add_signed(-1) {
                        avail = check_avail;
                    }
                },
                _ => {},
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut avail = 0;
        for col in 0..grid[0].len() {
            match grid[row][col] {
                'O' => {
                    if avail != col {
                        grid[row][avail] = 'O';
                        grid[row][col] = '.';
                    }
                    avail += 1;
                },
                '#' => {
                    avail = col + 1;
                },
                _ => {},
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut avail = grid[0].len() - 1;
        for col in (0..grid[0].len()).rev() {
            match grid[row][col] {
                'O' => {
                    if avail != col {
                        grid[row][avail] = 'O';
                        grid[row][col] = '.';
                    }
                    if let Some(check_avail) = avail.checked_add_signed(-1) {
                        avail = check_avail;
                    }
                },
                '#' => {
                    if let Some(check_avail) = col.checked_add_signed(-1) {
                        avail = check_avail;
                    }
                },
                _ => {},
            }
        }
    }
}

fn spin_cycle(mut grid: &mut Vec<Vec<char>>) {
    tilt_north(&mut grid);
    tilt_west(&mut grid);
    tilt_south(&mut grid);
    tilt_east(&mut grid);
}

fn hash_grid(grid: &Vec<Vec<char>>) -> String {
    grid
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<String>()
}

fn to_last_grid(mut grid: &mut Vec<Vec<char>>) {
    let cycles = 1_000_000_000;

    let mut seen: std::collections::HashMap<String, usize> = 
        std::collections::HashMap::new();

    for i in 1..=cycles {
        spin_cycle(&mut grid);
        let hash_key = hash_grid(grid);
        if let Some(first) = seen.get(&hash_key) {
            let cycle_len = i - first;
            let rem_cycles = (cycles - first) % cycle_len;
            for _ in 0..rem_cycles {
                spin_cycle(grid);
            }
            break;
        } else {
            seen.insert(hash_key, i.clone());
        }
    }
}

pub fn part_2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    to_last_grid(&mut grid);
    check_load(&grid)
}

#[cfg(test)]
mod tests {
    #[test]
    fn d14_p1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let res = super::part_1(input);
        println!("Expected: 136, Received: {res}");
        assert_eq!(136, res);
    }

    #[test]
    fn d14_p2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let res = super::part_2(input);
        println!("Expected: 64, Received: {res}");
        assert_eq!(64, res);
    }
}
