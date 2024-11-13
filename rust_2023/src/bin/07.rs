#[aoc::main(07)]
pub fn main(input: &str) -> (usize, usize) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

#[derive(Debug, std::cmp::PartialEq)]
enum HandType {
    FiveAKind(String),
    FourAKind(String),
    FullHouse(String),
    ThreeAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

pub fn part_1(input: &str) -> usize {
    let card_values: std::collections::HashMap<char, usize> = std::collections::HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', 9),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]);

    let mut lines = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| *l)
        .map(|l| {
            let parts = l
                .split_whitespace()
                .collect::<Vec<&str>>();

            let mut hand_map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
            parts[0].chars().for_each(|c| {
                let entry = hand_map.entry(c).or_insert(0);
                *entry += 1;
            });

            let hand_type = match hand_map.len() {
                1 => HandType::FiveAKind(parts[0].to_owned()),
                2 => {
                    let mut is_four_a_kind = false;
                    for (_, v) in hand_map {
                        if v == 4 {
                            is_four_a_kind = true;
                            break;
                        }
                    }
                    match is_four_a_kind {
                        true => HandType::FourAKind(parts[0].to_owned()),
                        false => HandType::FullHouse(parts[0].to_owned()),
                    }
                },
                3 => {
                    let mut is_three_a_kind = false;
                    for (_, v) in hand_map {
                        if v == 3 {
                            is_three_a_kind = true;
                        }
                    }
                    match is_three_a_kind {
                        true => HandType::ThreeAKind(parts[0].to_owned()),
                        false => HandType::TwoPair(parts[0].to_owned()),
                    }


                },
                4 => HandType::OnePair(parts[0].to_owned()),
                5 => HandType::HighCard(parts[0].to_owned()),
                _ => panic!("Not possible"),
            };

            let bid = parts[1].parse::<usize>().unwrap();

            (hand_type, bid)
        })
        .collect::<Vec<(HandType, usize)>>();

    lines
        .sort_by(|a, b| {
            if std::mem::discriminant(&a.0) == std::mem::discriminant(&b.0) {
                let a_val = get_hand(&a.0);
                let b_val = get_hand(&b.0);

                for (a_char, b_char) in a_val.chars().zip(b_val.chars()) {
                    let a_card_score = card_values.get(&a_char).unwrap();
                    let b_card_score = card_values.get(&b_char).unwrap();

                    if a_card_score > b_card_score {
                        return std::cmp::Ordering::Greater;
                    } else if b_card_score > a_card_score {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Equal
            } else {
                let a_strength = get_hand_strength(&a.0);
                let b_strength = get_hand_strength(&b.0);

                a_strength.cmp(&b_strength)
            }
        });

    lines
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| {
            let hand_total = hand.1 * (i + 1);
            acc + hand_total
        })
}

fn get_hand(hand_type: &HandType) -> &String {
    match hand_type {
        HandType::FiveAKind(val) => val,
        HandType::FourAKind(val) => val,
        HandType::FullHouse(val) => val,
        HandType::ThreeAKind(val) => val,
        HandType::TwoPair(val) => val,
        HandType::OnePair(val) => val,
        HandType::HighCard(val) => val,
    }
}

fn get_hand_strength(hand_type: &HandType) -> usize {
    match hand_type {
        HandType::FiveAKind(_) => 6,
        HandType::FourAKind(_) => 5,
        HandType::FullHouse(_) => 4,
        HandType::ThreeAKind(_) => 3,
        HandType::TwoPair(_) => 2,
        HandType::OnePair(_) => 1,
        HandType::HighCard(_) => 0,
    }

}

pub fn part_2(input: &str) -> usize {
    let card_values: std::collections::HashMap<char, usize> = std::collections::HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ]);

    let mut lines = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| *l)
        .map(|l| {
            let parts = l
                .split_whitespace()
                .collect::<Vec<&str>>();

            let mut hand_map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
            let mut joker_count = 0;
            parts[0].chars().for_each(|c| {
                if c == 'J' {
                    joker_count += 1;
                }
                let entry = hand_map.entry(c).or_insert(0);
                *entry += 1;
            });

            let hand_string = parts[0].to_owned();
            let hand_type = match hand_map.len() {
                1 => HandType::FiveAKind(hand_string),
                2 => {
                    let mut is_four_a_kind = false;
                    for (_, v) in hand_map {
                        if v == 4 {
                            is_four_a_kind = true;
                            break;
                        }
                    }
                    match is_four_a_kind {
                        true => HandType::FourAKind(hand_string),
                        false => HandType::FullHouse(hand_string),
                    }
                },
                3 => {
                    let mut is_three_a_kind = false;
                    for (_, v) in hand_map {
                        if v == 3 {
                            is_three_a_kind = true;
                        }
                    }
                    match is_three_a_kind {
                        true => HandType::ThreeAKind(hand_string),
                        false => HandType::TwoPair(hand_string),
                    }


                },
                4 => HandType::OnePair(hand_string),
                5 => HandType::HighCard(hand_string),
                _ => panic!("Not possible"),
            };

            let hand_type = match hand_type {
                HandType::FiveAKind(val) => HandType::FiveAKind(val),
                HandType::FourAKind(val) => {
                    match joker_count {
                        4 | 1 => HandType::FiveAKind(val),
                        _ => HandType::FourAKind(val)
                    }
                },
                HandType::FullHouse(val) => {
                    match joker_count {
                        3 | 2 => HandType::FiveAKind(val),
                        _ => HandType::FullHouse(val)
                    }
                },
                HandType::ThreeAKind(val) => {
                    match joker_count {
                        3 | 1 => HandType::FourAKind(val),
                        _ => HandType::ThreeAKind(val)
                    }
                },
                HandType::TwoPair(val) => {
                    match joker_count {
                        2 => HandType::FourAKind(val),
                        1 => HandType::FullHouse(val),
                        _ => HandType::TwoPair(val)
                    }
                },
                HandType::OnePair(val) => {
                    match joker_count {
                        2 | 1 => HandType::ThreeAKind(val),
                        _ => HandType::OnePair(val)

                    }
                },
                HandType::HighCard(val) => {
                    match joker_count {
                        1 => HandType::OnePair(val),
                        _ => HandType::HighCard(val)
                    }
                }
            };


            let bid = parts[1].parse::<usize>().unwrap();

            (hand_type, bid)
        })
        .collect::<Vec<(HandType, usize)>>();

    lines
        .sort_by(|a, b| {
            if std::mem::discriminant(&a.0) == std::mem::discriminant(&b.0) {
                let a_val = get_hand(&a.0);
                let b_val = get_hand(&b.0);

                for (a_char, b_char) in a_val.chars().zip(b_val.chars()) {
                    let a_card_score = card_values.get(&a_char).unwrap();
                    let b_card_score = card_values.get(&b_char).unwrap();

                    if a_card_score > b_card_score {
                        return std::cmp::Ordering::Greater;
                    } else if b_card_score > a_card_score {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Equal
            } else {
                let a_strength = get_hand_strength(&a.0);
                let b_strength = get_hand_strength(&b.0);

                a_strength.cmp(&b_strength)
            }
        });

    lines
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| {
            let hand_total = hand.1 * (i + 1);
            acc + hand_total
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn d7_p1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let res = super::part_1(input);
        println!("Expected: 6440, Received: {res}");
        assert_eq!(6440, res);
    }

    #[test]
    fn d7_p2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        let res = super::part_2(input);
        println!("Expected: 5905, Received: {res}");
        assert_eq!(5905, res);
    }
}
