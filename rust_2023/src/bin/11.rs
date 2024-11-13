use core::f64;

#[aoc::main(11)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut galaxies: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut non_empty_columns: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut non_empty_rows: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut empty_columns: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut empty_rows: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                non_empty_rows.insert(row);
                non_empty_columns.insert(col);
                galaxies.insert((row, col));
            }
        }
    }

    for row in 0..grid.len() {
        if non_empty_rows.get(&row).is_none() {
            empty_rows.insert(row);
        }
    }

    for col in 0..grid[0].len() {
        if non_empty_columns.get(&col).is_none() {
            empty_columns.insert(col);
        }
    }

    let galaxies = galaxies
        .iter()
        .map(|galaxy| galaxy.to_owned())
        .collect::<Vec<(usize, usize)>>();

    let galaxies = galaxies
        .iter()
        .map(|galaxy| {
            let row_count = empty_rows
                .iter()
                .filter(|r| *r < &galaxy.0)
                .count();

            let col_count = empty_columns
                .iter()
                .filter(|c| *c < &galaxy.1)
                .count();

            (galaxy.0 + row_count, galaxy.1 + col_count)
        })
        .collect::<Vec<(usize, usize)>>();

    let mut pairs: Vec<((f64, f64), (f64, f64))> = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxies_i = (galaxies[i].0 as f64, galaxies[i].1 as f64);
            let galaxies_j = (galaxies[j].0 as f64, galaxies[j].1 as f64);
            pairs.push((galaxies_i, galaxies_j));
        }
    }

    let mut total = 0;
    for (start, target) in pairs {
        let slope: f64 = (target.1 - start.1) / (target.0 - start.0);
        let slop_inv: f64 = (target.0 - start.0) / (target.1 - start.1);
        let scaled_x = f64::sqrt(1f64 + slope * slope);
        let scaled_y = f64::sqrt(1f64 + slop_inv * slop_inv);

        let mut x_ray_len = 0f64;
        let mut y_ray_len = 0f64;
        let mut curr_point = start.clone();
        let mut steps = 0;
        while curr_point != target {
            let pot_x = x_ray_len + scaled_x;
            let pot_y = y_ray_len + scaled_y;

            if pot_x < pot_y {
                x_ray_len = pot_x;
                if target.0 < start.0 {
                    curr_point = (curr_point.0 - 1f64, curr_point.1);
                } else {
                    curr_point = (curr_point.0 + 1f64, curr_point.1);
                }
            } else {
                y_ray_len = pot_y;
                if target.1 < start.1 {
                    curr_point = (curr_point.0, curr_point.1 - 1f64);
                } else {
                    curr_point = (curr_point.0, curr_point.1 + 1f64);
                }
            }

            steps += 1;
        }

        total += steps;
    }
    
    total
}

pub fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut galaxies: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut non_empty_columns: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut non_empty_rows: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut empty_columns: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut empty_rows: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                non_empty_rows.insert(row);
                non_empty_columns.insert(col);
                galaxies.insert((row, col));
            }
        }
    }

    for row in 0..grid.len() {
        if non_empty_rows.get(&row).is_none() {
            empty_rows.insert(row);
        }
    }

    for col in 0..grid[0].len() {
        if non_empty_columns.get(&col).is_none() {
            empty_columns.insert(col);
        }
    }

    let galaxies = galaxies
        .iter()
        .map(|galaxy| galaxy.to_owned())
        .collect::<Vec<(usize, usize)>>();

    let galaxies = galaxies
        .iter()
        .map(|galaxy| {
            let row_count = empty_rows
                .iter()
                .filter(|r| *r < &galaxy.0)
                .count();
    
            let col_count = empty_columns
                .iter()
                .filter(|c| *c < &galaxy.1)
                .count();
    
            let row_count = if row_count > 0 { row_count * (1000000 - 1) } else { 0 };
            let col_count = if col_count > 0 { col_count * (1000000 - 1) } else { 0 };
    
            (galaxy.0 + row_count, galaxy.1 + col_count)
        })
        .collect::<Vec<(usize, usize)>>();

    let mut pairs: Vec<((f64, f64), (f64, f64))> = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxies_i = (galaxies[i].0 as f64, galaxies[i].1 as f64);
            let galaxies_j = (galaxies[j].0 as f64, galaxies[j].1 as f64);
            pairs.push((galaxies_i, galaxies_j));
        }
    }

    let mut total = 0;
    for (start, target) in pairs {
        let dist: usize = (target.0 - start.0).abs() as usize + (target.1 - start.1).abs() as usize;
        total += dist;
    }
    
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn d11_p1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let res = super::part_1(input);
        println!("Expected 374, Received: {res}");
        assert_eq!(374, res);
    }

    #[test]
    fn d11_p2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let res = super::part_2(input);
        println!("Expected 1030, Received: {res}");
        assert_eq!(1030, res);
    }
    
}
