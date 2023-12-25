#[aoc::main(21)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

struct Grid {
    pub row_max: usize,
    pub col_max: usize,
    pub start: (usize, usize),
    pub grid: Vec<Vec<char>>,
    pub target_steps: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>, target_steps: usize) -> Self {
        let row_max = grid.len();
        let col_max = grid[0].len();

        let mut start = (0, 0);
        'outer: for r in 0..row_max {
            for c in 0..col_max {
                if grid[r][c] == 'S' {
                    start = (r, c);
                    break 'outer;
                }
            }
        }

        Self {
            row_max,
            col_max,
            start,
            grid,
            target_steps,
        }
    }

    fn naive_traverse_and_count(&self) -> usize {
        let mut queue = std::collections::VecDeque::new();
        let mut seen = std::collections::HashSet::new();

        queue.push_back((self.start.0, self.start.1, 0));

        while let Some((r, c, lv)) = queue.pop_front() {
            if self.grid[r][c] == '#' || seen.contains(&(r, c, lv)) {
                continue;
            }

            seen.insert((r, c, lv));
            if lv == self.target_steps {
                continue;
            }

            if r > 0 {
                queue.push_back((r - 1, c, lv + 1));
            }
            if r < self.row_max - 1 {
                queue.push_back((r + 1, c, lv + 1));
            }
            if c > 0 {
                queue.push_back((r, c - 1, lv + 1));
            }
            if c < self.col_max - 1 {
                queue.push_back((r, c + 1, lv + 1));
            }
        }

        seen
            .iter()
            .filter(|(_, _, lv)| lv == &self.target_steps)
            .count()
    }
}

pub fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let grid = Grid::new(grid, 64);
    let steps = grid.naive_traverse_and_count();
        
    steps
}

pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn d21_p1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        let res = super::part_1(input);
        println!("Expected: 16, Received: {res}");
        assert_eq!(16, res);
    }

    #[test]
    fn d21_p2() {
        let input = "";

        let res = super::part_2(input);
        println!("Expected: , Received: {res}");
        assert_eq!(0, res);
    }
}
