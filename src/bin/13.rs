#[aoc::main(13)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

fn find_left_of_mirror(chunk: &Vec<Vec<char>>) -> usize {
    let mut possible_mirrors = std::collections::HashSet::from_iter(
        (1..chunk[0].len()).collect::<Vec<usize>>());

    for line in chunk {
        possible_mirrors = find_vert_mirror_idx(&line, &possible_mirrors);
    }

    let mirror_point: usize = possible_mirrors
        .iter()
        .nth(0)
        .unwrap_or(&0usize)
        .to_owned()
        .into();

    mirror_point
}

fn find_vert_mirror_idx(line: &Vec<char>, possible_points: &std::collections::HashSet<usize>) -> std::collections::HashSet<usize> {
    let mut mirror_points: std::collections::HashSet<usize> = std::collections::HashSet::new();

    'outer: for possible_mirror in possible_points {
        let mut l = possible_mirror.clone();
        let mut r = possible_mirror.clone();

        while l.checked_add_signed(-1).is_some() && r < line.len() {
            l = l.checked_add_signed(-1).unwrap();
            if line[l] != line[r] {
                continue 'outer;
            }
            r += 1;
        }

        mirror_points.insert(possible_mirror.clone());
    }

    mirror_points
}

pub fn part_1(input: &str) -> usize {
    let blocks = input
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut total = 0;
    for block in blocks {
        let chunk = block
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let vert = find_left_of_mirror(&chunk);
        if vert > 0 {
            total += vert;
            continue;
        }

        let mut rotated: Vec<Vec<char>> = Vec::new();
        for j in 0..chunk[0].len() {
            let mut rotated_row: Vec<char> = Vec::new();
            for i in 0..chunk.len() {
                rotated_row.push(chunk[i][j]);
            }
            rotated.push(rotated_row);
        }

        let horiz = find_left_of_mirror(&rotated) * 100;
        total += horiz;
    }

    total
}

fn check_all_rows(chunk: &Vec<Vec<char>>) -> usize {
    'outer: for i in 1..chunk[0].len() {
        let mut l = i.clone();
        let mut r = i.clone();
        let mut diffs = 0;

        while l.checked_add_signed(-1).is_some() && r < chunk[0].len() {
            l = l.checked_add_signed(-1).unwrap();
            for j in 0..chunk.len() {
                if chunk[j][l] != chunk[j][r] {
                    diffs += 1;
                }
                if diffs > 1 {
                    continue 'outer;
                }
            }
            r += 1;
        }

        if diffs == 1 {
            return i;
        }
    }

    0
}

pub fn part_2(input: &str) -> usize {
    let blocks = input
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut total = 0;
    for block in blocks {
        let chunk = block
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut rotated: Vec<Vec<char>> = Vec::new();
        for j in 0..chunk[0].len() {
            let mut rotated_row: Vec<char> = Vec::new();
            for i in 0..chunk.len() {
                rotated_row.push(chunk[i][j]);
            }
            rotated.push(rotated_row);
        }

        let vert = check_all_rows(&chunk);
        let horiz = check_all_rows(&rotated);

        total += vert + horiz * 100;
    }

    total
}

#[cfg(test)]
pub mod test {
    #[test]
    fn d13_p1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let res = super::part_1(input);
        println!("Expected: 405, Received: {res}");
        assert_eq!(405, res);
    }

    #[test]
    fn d13_p2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let res = super::part_2(input);
        println!("Expected: 400, Received: {res}");
        assert_eq!(400, res);
    }
}
