#[aoc::main(12)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

struct Row {
    line: Vec<char>,
    groups: Vec<usize>,
    cache: std::collections::HashMap<(usize, usize, usize), usize>,
}

pub fn part_1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            let line: (&str, &str) = line
                .split_once(' ')
                .unwrap()
                .into();

            let groups = line.1
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let line = line.0.chars().collect::<Vec<char>>();

            Row {
                line,
                groups,
                cache: std::collections::HashMap::new(),
            }
        })
        .collect::<Vec<Row>>();

    let mut total = 0;
    for mut line in lines {
        total += line.count_arrangements(0, 0, 0);
    }

    total
}

impl Row {
    fn count_arrangements(& mut self, group_idx: usize, line_idx: usize, curr_chunk_size: usize) -> usize {
        let cache_key = (group_idx, line_idx, curr_chunk_size);
        if let Some(cache_hit) = self.cache.get(&cache_key) {
            return *cache_hit;
        }

        if line_idx == self.line.len() {
            if curr_chunk_size == 0 && group_idx == self.groups.len() {
                return 1;
            }
            if group_idx == self.groups.len() - 1 && curr_chunk_size == self.groups[group_idx] {
                return 1;
            }
            return 0;
        }

        let arrangements = match &self.line[line_idx] {
            '#' => self.count_arrangements(group_idx, line_idx + 1, curr_chunk_size + 1),
            '.' => {
                if curr_chunk_size == 0 {
                    self.count_arrangements(group_idx, line_idx + 1, 0)
                } else if group_idx < self.groups.len() && self.groups[group_idx] == curr_chunk_size {
                    self.count_arrangements(group_idx + 1, line_idx + 1, 0)
                } else {
                    0
                }
            },
            '?' => {
                let count_if_damaged = self.count_arrangements(group_idx, line_idx + 1, curr_chunk_size + 1);
                let count_if_op_after_op = if curr_chunk_size == 0 { 
                    self.count_arrangements(group_idx, line_idx + 1, 0) 
                } else { 
                    0 
                };
                let count_if_op_after_dmg = if group_idx < self.groups.len() && self.groups[group_idx] == curr_chunk_size { 
                    self.count_arrangements(group_idx + 1, line_idx + 1, 0)
                } else { 
                    0 
                };

                count_if_damaged + count_if_op_after_dmg + count_if_op_after_op
            },
            _ => panic!("Invalid character"),
        };

        self.cache.insert(cache_key, arrangements);

        arrangements
    }
}

pub fn part_2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            let line: (&str, &str) = line
                .split_once(' ')
                .unwrap()
                .into();

            let mut expanded_groups = format!("{},", line.1);
            let mut expanded_line = format!("{}?", line.0);
            for i in 0..4 {
                expanded_groups.push_str(line.1);
                expanded_line.push_str(line.0);
                if i != 3 {
                    expanded_groups.push(',');
                    expanded_line.push('?');
                }
            }

            let groups = expanded_groups
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let line = expanded_line.chars().collect::<Vec<char>>();

            Row {
                line,
                groups,
                cache: std::collections::HashMap::new(),
            }
        })
        .collect::<Vec<Row>>();


    let mut total = 0;
    for mut line in lines {
        total += line.count_arrangements(0, 0, 0);
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn d12_p1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let res = super::part_1(input);
        println!("Expected: 21, Received: {res}");
        assert_eq!(21, res);
    }

    #[test]
    fn d12_p2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let res = super::part_2(input);
        println!("Expected: 525152, Received: {res}");
        assert_eq!(525152, res);
    }
}
