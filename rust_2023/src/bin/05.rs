#[aoc::main(05)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

pub fn part_1(input: &str) -> usize {
    let mut lines = input
        .lines()
        .collect::<Vec<&str>>();
    lines.push("");

    let mut seeds = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| {
            let num = num.trim();
            num.parse::<usize>().unwrap()
        })
        .collect::<Vec<usize>>();

    let mut ranges_map: Vec<Vec<(std::ops::Range<usize>, usize)>> = Vec::new();
    let mut curr_map: Vec<(std::ops::Range<usize>, usize)> = Vec::new();
    lines
        .iter()
        .skip(2)
        .for_each(|l| {
            if l.is_empty() {
                ranges_map.push(curr_map.clone());
                curr_map = Vec::new();
            } else if l.chars().collect::<Vec<char>>()[0].is_ascii_digit() {
                let nums = l
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let size = nums[nums.len() - 1];
                let in_range = nums[1]..(nums[1] + size);
                let out_start = nums[0];

                curr_map.push((in_range, out_start));
            }
        });

    for range_group in ranges_map {
        for seed in &mut seeds {
            for range in &range_group {
                if range.0.contains(&seed) {
                    *seed = *seed - range.0.start + range.1;
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let chunks = input
        .split("\n\n")
        .collect::<Vec<&str>>();

    let seeds = chunks[0]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0].parse::<usize>().unwrap();
            let size = chunk[1].parse::<usize>().unwrap();
            
            start..(start + size)
        })
        .collect::<Vec<std::ops::Range<usize>>>();

    let ranges = chunks[1..]
        .iter()
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|line| {
                    let line = line
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    
                    let size = line[line.len() - 1];
                    let in_range = line[1]..(line[1] + size);
                    let out_start = line[0];

                    (in_range, out_start)
                })
                .collect::<Vec<(std::ops::Range<usize>, usize)>>()
        })
        .collect::<Vec<Vec<(std::ops::Range<usize>, usize)>>>();

    let mut output: Vec<usize> = Vec::new();

    for seed_group in seeds {
        for mut seed in seed_group {
            for range_group in &ranges {
                for range in range_group {
                    if range.0.contains(&seed) {
                        seed = seed - range.0.start + range.1;
                        break;
                    }
                }
            }

            output.push(seed);
        }
    }
    
    *output.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn d5_p1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let res = super::part_1(input);
        println!("Expected: 35 , Received: {res}");
        assert_eq!(35, res);
    }

    #[test]
    fn d5_p2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let res = super::part_2(input);
        println!("Expected: 46, Received: {res}");
        assert_eq!(46, res);
    }
}
