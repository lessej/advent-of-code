#[aoc::main(02)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);
    (p1,p2)
}

fn part_1(input: &str) -> usize {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let lines: Vec<&str> = lines[..lines.len() - 1].into();
    let max_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::from([
        ("blue", 14),
        ("red", 12),
        ("green", 13),
    ]);
    let mut id_total = 0;

    'line_loop: for (i, line) in lines.iter().enumerate() {
        let id_removed = line[7..].to_string();
        let groups = id_removed.split(';').collect::<Vec<&str>>();

        for group in groups {
            let colors = group.split(',').collect::<Vec<&str>>();

            for color in colors {
                let parts = color.split(' ').collect::<Vec<&str>>();
                let count = parts[1].parse::<usize>().unwrap();
                let color_name = parts[2];

                if let Some(max_count) = max_counts.get(color_name) {
                    if &count > max_count {
                        continue 'line_loop;
                    }
                }
            }
        }

        id_total += i + 1;
    }

    id_total
} 

fn part_2(input: &str) -> usize {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let lines: Vec<&str> = lines[..lines.len() - 1].into();
    let mut min_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::from([
        ("blue", std::usize::MIN),
        ("red", std::usize::MIN),
        ("green", std::usize::MIN),
    ]);

    let mut power_sum_total = 0;

    for line in lines.iter() {
        let id_removed = line[7..].to_string();
        let groups = id_removed.split(';').collect::<Vec<&str>>();

        for group in groups {
            let colors = group.split(',').collect::<Vec<&str>>();

            for color in colors {
                let parts = color.split(' ').collect::<Vec<&str>>();
                let count = parts[1].parse::<usize>().unwrap();
                let color_name = parts[2];

                if let Some(min_count) = min_counts.get_mut(color_name) {
                    if &count > min_count {
                        *min_count = count;
                    }

                }
            }
        }

        let blue_count = min_counts.get("blue").unwrap();
        let red_count = min_counts.get("red").unwrap();
        let green_count = min_counts.get("green").unwrap();
        let line_power_sum = blue_count * red_count * green_count;

        min_counts = std::collections::HashMap::from([
            ("blue", std::usize::MIN),
            ("red", std::usize::MIN),
            ("green", std::usize::MIN),
        ]);

        power_sum_total += line_power_sum;
    }

    power_sum_total
} 

#[cfg(test)]
mod tests {
    #[test]
    fn d2_p1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let res = super::part_1(input);
        println!("Expected: 8, Given: {res}");
        assert_eq!(8, res);
    }

    #[test]
    fn d2_p2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let res = super::part_2(input);
        println!("Expected: 2286, Given: {res}");
        assert_eq!(2286, res);
    }
}
