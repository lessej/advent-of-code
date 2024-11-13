#[aoc::main(04)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let lines = input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| *l)
        .collect::<Vec<&str>>();

    let mut total = 0;
    let mut winning_nums: std::collections::HashSet<usize> = std::collections::HashSet::new();


    for line in lines {
        let line_parts = line
            .split('|')
            .collect::<Vec<&str>>();

        line_parts[0]
            .split(':')
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|num| !num.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .for_each(|num| { winning_nums.insert(num); });

        let line_total: usize = line_parts[1]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|num| num.trim())
            .filter(|num| !num.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .filter(|num| winning_nums.contains(num))
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });

        winning_nums = std::collections::HashSet::new();

        total += line_total;
    }

    total
}

pub fn part_2(input: &str) -> usize {
    let lines = input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| *l)
        .collect::<Vec<&str>>();

    let mut winning_nums: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut row_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let line_parts = line
            .split('|')
            .collect::<Vec<&str>>();

        line_parts[0]
            .split(':')
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|num| !num.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .for_each(|num| { winning_nums.insert(num); });

        let winning_count: usize = line_parts[1]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|num| num.trim())
            .filter(|num| !num.is_empty())
            .map(|num| num.parse::<usize>().unwrap())
            .filter(|num| winning_nums.contains(num))
            .count();

        winning_nums = std::collections::HashSet::new();

        let curr_row = row_counts.entry(i).or_insert(1).clone();
        (i + 1..=(i + winning_count)).for_each(|line_num| {
            let future_row = row_counts.entry(line_num).or_insert(1);
            *future_row += curr_row;
        });
    }

    row_counts.iter().fold(0, |acc, (_, count)| acc + count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn d4_p1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = super::part_1(input);
        println!("Expected: 13, Received: {res}");
        assert_eq!(13, res);
    }

    #[test]
    fn d4_p2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = super::part_2(input);
        println!("Expected: 30, Received: {res}");
        assert_eq!(30, res);

    }
}
