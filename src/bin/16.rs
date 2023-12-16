#[aoc::main(16)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

struct Grid {
    pub grid: Vec<Vec<u8>>,
    pub energized: std::collections::HashSet<(usize, usize, Dir)>
}

#[derive(Debug, Clone, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash)]
enum Dir {
    L,
    R,
    U,
    D,
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        Self {
            grid,
            energized: std::collections::HashSet::new()
        }
    }

    fn reset(&mut self) {
        self.energized = std::collections::HashSet::new();
    }

    fn total(&self) -> usize {
        let mut count_hash: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        for (row, col, _) in &self.energized {
            count_hash.insert((row.clone(), col.clone()));
        }
        count_hash.iter().count()
    }

    fn trace(&mut self, row: i64, col: i64, dir: Dir) {
        if row < 0 || row >= self.grid.len() as i64 || col < 0 || col >= self.grid[0].len() as i64 {
            return;
        }

        if self.energized.get(&(row as usize, col as usize, dir.clone())).is_some() {
            return;
        }

        self.energized.insert((row as usize, col as usize, dir.clone()));

        match self.grid[row as usize][col as usize] {
            b'|' => {
                match dir {
                    Dir::L | Dir::R => {
                        self.trace(row - 1, col, Dir::U);
                        self.trace(row + 1, col, Dir::D);
                    },
                    Dir::U => self.trace(row - 1, col, dir),
                    Dir::D => self.trace(row + 1, col, dir),
                }
            },
            b'-' => {
                match dir {
                    Dir::L => self.trace(row, col - 1, dir),
                    Dir::R => self.trace(row, col + 1, dir),
                    Dir::U | Dir::D => {
                        self.trace(row, col - 1, Dir::L);
                        self.trace(row, col + 1, Dir::R);
                    }
                }
            },
            b'\\' => {
                match dir {
                    Dir::L => self.trace(row - 1, col, Dir::U),
                    Dir::R => self.trace(row + 1, col, Dir::D),
                    Dir::U => self.trace(row, col - 1, Dir::L),
                    Dir::D => self.trace(row, col + 1, Dir::R),
                }
            },
            b'/' => {
                match dir {
                    Dir::L => self.trace(row + 1, col, Dir::D),
                    Dir::R => self.trace(row - 1, col, Dir::U),
                    Dir::U => self.trace(row, col + 1, Dir::R),
                    Dir::D => self.trace(row, col - 1, Dir::L),
                }
            },
            b'.' => {
                match dir {
                    Dir::L => self.trace(row, col - 1, dir),
                    Dir::R => self.trace(row, col + 1, dir),
                    Dir::U => self.trace(row - 1, col, dir),
                    Dir::D => self.trace(row + 1, col, dir),
                }
            },
            _ => panic!("unexpected char"),
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l
            .as_bytes()
            .iter()
            .map(|b| b.to_owned())
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut grid = Grid::new(grid);
    grid.trace(0, 0, Dir::R);

    grid.total()
}

pub fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l
            .as_bytes()
            .iter()
            .map(|b| b.to_owned())
            .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();


    let mut starts: Vec<(usize, usize, Dir)> = Vec::new();
    for i in 0..grid[0].len() {
        starts.push((0, i, Dir::D));
        starts.push((grid.len() - 1, i, Dir::U));
    }
    for i in 1..grid.len() - 2 {
        starts.push((i, 0, Dir::R));
        starts.push((i, grid[0].len() - 1, Dir::L));
    }
    starts.push((0, 0, Dir::R));
    starts.push((0, grid[0].len() - 1, Dir::L));
    starts.push((grid.len() - 1, 0, Dir::R));
    starts.push((grid.len() - 1, grid[0].len() - 1, Dir::L));

    let mut grid = Grid::new(grid);
    let mut max = 0;
    for (row, col, dir) in starts {
        grid.trace(row as i64, col as i64, dir);
        let total = grid.total();
        if total > max {
            max = total;
        }
        grid.reset();
    }

    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn d16_p1() {
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        let res = super::part_1(input);
        println!("Expected: 46, Received: {res}");
        assert_eq!(46, res);
    }

    #[test]
    fn d16_p2() {
        let input = "";

        let res = super::part_2(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }
}

