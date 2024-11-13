#[aoc::main(17)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Dir {
    U,
    D,
    L,
    R,
    No,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Vertex {
    pos: (usize, usize),
    dir: Dir,
    dir_count: usize,
}

impl Vertex {
    fn start(row: usize, col: usize) -> Self {
        Self { 
            pos: (row, col), 
            dir: Dir::No,
            dir_count: 0,
        }
    }

    fn new(parent: &Vertex, dir: Dir) -> Self {
        let mut new_vert = parent.clone();
        if dir == parent.dir {
            new_vert.dir_count += 1;
        } else {
            new_vert.dir = dir.clone();
            new_vert.dir_count = 1;
        }

        match dir {
            Dir::U => new_vert.pos.0 -= 1,
            Dir::D => new_vert.pos.0 += 1,
            Dir::L => new_vert.pos.1 -= 1,
            Dir::R => new_vert.pos.1 += 1,
            Dir::No => panic!("Unexpected direction"),
        };

        new_vert
    }

    fn get_adjacent(&self, max_row: usize, max_col: usize, min_rep: usize, max_rep: usize) -> Vec<Vertex> {
        let mut adj = Vec::new();
        let (r, c) = self.pos;

        if self.dir == Dir::No {
            if c > 0 {
                adj.push(Vertex::new(&self, Dir::L));
            }
            if c < max_col - 1 {
                adj.push(Vertex::new(&self, Dir::R));
            }
            if r > 0 {
                adj.push(Vertex::new(&self, Dir::U));
            }
            if r < max_row - 1 {
                adj.push(Vertex::new(&self, Dir::D));
            }

            return adj;
        }

        if self.dir_count >= min_rep {
            match self.dir {
                Dir::U | Dir::D => {
                    if c > 0 {
                        adj.push(Vertex::new(&self, Dir::L));
                    }
                    if c < max_col - 1 {
                        adj.push(Vertex::new(&self, Dir::R));
                    }
                },
                Dir::R | Dir::L => {
                    if r > 0 {
                        adj.push(Vertex::new(&self, Dir::U));
                    }
                    if r < max_row - 1 {
                        adj.push(Vertex::new(&self, Dir::D));
                    }
                },
                _ => panic!("Unexpected direction"),
            }
        }

        if self.dir_count < max_rep {
            match self.dir {
                Dir::U => if r > 0 {
                    adj.push(Vertex::new(&self, self.dir.clone()));
                },
                Dir::D => if r < max_row - 1 {
                    adj.push(Vertex::new(&self, self.dir.clone()));
                },
                Dir::L => if c > 0 {
                    adj.push(Vertex::new(&self, self.dir.clone()));
                },
                Dir::R => if c < max_col - 1 {
                    adj.push(Vertex::new(&self, self.dir.clone()));
                },
                Dir::No => panic!("Unexpected direction"),
            }

        }

        adj
    }
}

fn dijkstra(grid: &Vec<Vec<usize>>, start: Vertex, min_rep: usize, max_rep: usize) -> usize {
    let max_row = grid.len();
    let max_col = grid[0].len();
    let mut heap = std::collections::BinaryHeap::new();
    let mut cost = std::collections::HashMap::new();
    let mut seen = std::collections::HashSet::new();
    let mut min = std::usize::MAX;

    cost.insert(start, 0);
    heap.push(std::cmp::Reverse((0, start)));

    while let Some(std::cmp::Reverse((_, v))) = heap.pop() {
        seen.insert(v);

        for adj in v.get_adjacent(max_row, max_col, min_rep, max_rep) {
            if !seen.contains(&adj) {
                let new_cost = if let Some(c) = cost.get(&v) {
                    c + grid[adj.pos.0][adj.pos.1]
                } else {
                    std::usize::MAX
                };

                if new_cost < *cost.get(&adj).unwrap_or(&std::usize::MAX) {
                    if adj.pos == (max_row - 1, max_col - 1) {
                        min = min.min(new_cost);
                    }
                    cost.insert(adj, new_cost);
                    heap.push(std::cmp::Reverse((new_cost, adj)));
                }
            }
        }
    }

    min
}

pub fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        )
        .collect();

    let min = dijkstra(&grid, Vertex::start(0, 0), 0, 3);

    min
}

pub fn part_2(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        )
        .collect();

    let min = dijkstra(&grid, Vertex::start(0, 0), 4, 10);

    min
}

#[cfg(test)]
mod tests {
    #[test]
    fn d17_p1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let res = super::part_1(input);
        println!("Expected: 102, Received: {res}");
        assert_eq!(102, res);
    }

    #[test]
    fn d17_p2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let res = super::part_2(input);
        println!("Expected: 94, Received: {res}");
        assert_eq!(94, res);
    }
}
