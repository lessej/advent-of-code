#[aoc::main(18)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

#[derive(Clone, Eq, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" | "3" => Dir::U,
            "D" | "1" => Dir::D,
            "L" | "2" => Dir::L,
            "R" | "0" => Dir::R,
            _ => panic!("Unexpected direction: {value}"),
        }
    }
}

struct Instruction {
    pub dir: Dir,
    pub size: usize,
}

struct Grid {
    pub grid: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
    pub start: (usize, usize),
    pub outer: usize,
}

impl Grid {
    fn new(height: (isize, isize), width: (isize, isize)) -> Self {
        let start = (height.0.abs() as usize, width.0.abs() as usize);
        let height = (height.1 - height.0).abs() as usize + 1;
        let width = (width.1 - width.0).abs() as usize + 1;
        let grid = vec![vec!['.'; width]; height];
        
        Self {
            grid,
            height,
            width,
            start,
            outer: 0,
        }
    }

    fn color_outline(&mut self, instructions: &Vec<Instruction>) {
        let mut curr_pos = self.start;

        for Instruction { dir, size } in instructions {
            match dir {
                Dir::U => {
                    for i in (curr_pos.0 - size)..curr_pos.0 {
                        self.grid[i][curr_pos.1] = '#';
                    }
                    curr_pos.0 -= size;
                },
                Dir::D => {
                    for i in curr_pos.0..=(curr_pos.0 + size) {
                        self.grid[i][curr_pos.1] = '#';
                    }
                    curr_pos.0 += size;
                },
                Dir::L => {
                    for i in (curr_pos.1 - size)..curr_pos.1 {
                        self.grid[curr_pos.0][i] = '#';
                    }
                    curr_pos.1 -= size;
                },
                Dir::R => {
                    for i in curr_pos.1..=(curr_pos.1 + size) {
                        self.grid[curr_pos.0][i] = '#';
                    }
                    curr_pos.1 += size;
                },
            }
        }
    }

    fn count_outer(&mut self) -> usize {
        for r in 0..self.height {
            self.recurse(r, 0);
            self.recurse(r, self.width - 1);
        }

        for c in 0..self.width {
            self.recurse(0, c);
            self.recurse(self.height - 1, c);
        }

        (self.height * self.width) - self.outer
    }

    fn recurse(&mut self, r: usize, c: usize) {
        if self.grid[r][c] == '#' || self.grid[r][c] == '!' {
            return;
        }

        self.grid[r][c] = '!';
        self.outer += 1;

        if r > 0 {
            self.recurse(r - 1, c);
        }
        if r < self.height - 1 {
            self.recurse(r + 1, c);
        }
        if c > 0 {
            self.recurse(r, c - 1);
        }
        if c < self.width - 1 {
            self.recurse(r, c + 1)
        }
    }
}

fn get_ranges(instructions: &Vec<Instruction>) -> ((isize, isize), (isize, isize)) {
    let mut height_range: (isize, isize) = (0, 0);
    let mut width_range: (isize, isize) = (0, 0);
    let mut curr_pos: (isize, isize) = (0, 0);

    for instr in instructions {
        match instr.dir {
            Dir::U => curr_pos.0 -= instr.size as isize,
            Dir::D => curr_pos.0 += instr.size as isize,
            Dir::L => curr_pos.1 -= instr.size as isize,
            Dir::R => curr_pos.1 += instr.size as isize,
        }

        if curr_pos.0 < height_range.0 {
            height_range.0 = curr_pos.0;
        }
        if curr_pos.0 > height_range.1 {
            height_range.1 = curr_pos.0;
        }
        if curr_pos.1 < width_range.0 {
            width_range.0 = curr_pos.1;
        }
        if curr_pos.1 > width_range.1 {
            width_range.1 = curr_pos.1;
        }
    }

    (height_range, width_range)
}

fn get_coords(instructions: &Vec<Instruction>) -> Vec<(isize, isize)> {
    let mut coords = Vec::new();
    let mut curr_pos: (isize, isize) = (0, 0);

    for instr in instructions {
        match instr.dir {
            Dir::U => curr_pos.0 -= instr.size as isize,
            Dir::D => curr_pos.0 += instr.size as isize,
            Dir::L => curr_pos.1 -= instr.size as isize,
            Dir::R => curr_pos.1 += instr.size as isize,
        }

        coords.push(curr_pos.clone());
    }

    coords
}

fn shoelace(coords: &Vec<(isize, isize)>) -> usize {
    let mut sum_l = 0;
    for (x, y) in coords.iter().zip(coords.iter().skip(1)) {
        sum_l += x.0 * y.1;
    }
    sum_l += coords[coords.len() - 1].0 * coords[0].1;
    let mut sum_r = 0;
    for (x, y) in coords.iter().skip(1).zip(coords.iter()) {
        sum_r += x.0 * y.1
    }
    sum_r += coords[0].0 * coords[coords.len() -1].1;

    ((sum_l - sum_r) / 2).abs() as usize
}

fn perim(instructions: &Vec<Instruction>) -> usize {
    instructions.iter().fold(0, |acc, instr| acc + instr.size)
}

pub fn part_1(input: &str) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l
                .split_whitespace()
                .collect();

            Instruction {
                dir: Dir::from(l[0]),
                size: l[1].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let (height_range, width_range) = get_ranges(&instructions);
    let mut grid = Grid::new(height_range, width_range);
    grid.color_outline(&instructions);

    grid.count_outer()
}

pub fn part_2(input: &str) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split_whitespace().collect();
            let hex_val: &str = &l[2][2..=6];
            let dir = Dir::from(&l[2][7..=7]);
            let hex_val = usize::from_str_radix(hex_val, 16).unwrap();

            Instruction {
                dir,
                size: hex_val,
            }
        })
        .collect();

    let coords = get_coords(&instructions);
    let area = shoelace(&coords);
    let perim = perim(&instructions);

    area + 1 + (perim / 2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn d18_p1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let res = super::part_1(input);
        println!("Expected: , Received: {res}");
        assert_eq!(62, res);
    }

    #[test]
    fn d18_p2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let res = super::part_2(input);
        println!("Expected: 952408144115, Received: {res}");
        assert_eq!(952408144115, res);
    }
}
