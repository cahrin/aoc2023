use std::collections::HashSet;

use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(4);
    println!("Running Day 4");
    println!(
        "Part 1 :: How many points are they worth in total? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: How many total scratchcards do you end up with? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    // Parse input into `Card`s, get each card value and sum
    input_to_cards(input)
        .iter()
        .fold(0, |sum, card| sum + card.value())
}

fn part2(input: &str) -> usize {
    let cards = input_to_cards(input);
    // Start with a card count of 1 for each
    let mut card_counts = vec![1; cards.len()];

    // For each card get the number of winning matches and increment subsequent card
    // counts based on the number of winning matches. Do that step once for every version
    // of the card, tracked by `card_counts`.
    for (i, card) in cards.iter().enumerate() {
        for _ in 0..card_counts[i] {
            for j in 1..=card.winning_number_matches().len() {
                card_counts[i + j] += 1;
            }
        }
    }

    card_counts.iter().sum()
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    numbers_card_has: Vec<usize>,
}

impl Card {
    fn winning_number_matches(&self) -> HashSet<usize> {
        let mut matches: HashSet<usize> = HashSet::new();
        for winning_num in self.winning_numbers.iter() {
            if self.numbers_card_has.contains(winning_num) {
                matches.insert(*winning_num);
            }
        }
        matches
    }

    fn value(&self) -> usize {
        let matches = self.winning_number_matches();
        if matches.is_empty() {
            0
        } else {
            2_usize.pow(matches.len() as u32 - 1)
        }
    }
}

fn input_to_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            line.split(": ").last().unwrap().split(" | ").fold(
                Card {
                    winning_numbers: vec![],
                    numbers_card_has: vec![],
                },
                |mut card, part| {
                    if card.winning_numbers.is_empty() {
                        card.winning_numbers = numbers_string_to_number_vec(part);
                    } else {
                        card.numbers_card_has = numbers_string_to_number_vec(part);
                    }
                    card
                },
            )
        })
        .collect()
}

fn numbers_string_to_number_vec(numbers: &str) -> Vec<usize> {
    numbers
        .split_whitespace()
        .map(|num| {
            num.trim()
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("Couldn't parse number from '{num}' : {e}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day04::{numbers_string_to_number_vec, part1, part2};

    #[test]
    fn test_numbers_string_to_number_vec() {
        assert_eq!(
            vec![1, 2, 3, 4],
            numbers_string_to_number_vec("  1     2 3     4")
        );
        assert_eq!(
            vec![9, 31, 6, 82, 29, 41, 67, 75, 63, 10, 9],
            numbers_string_to_number_vec(" 9 31  6 82 29 41 67 75 63 10  9")
        );
    }

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(
            13,
            part1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
        )
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(
            30,
            part2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
        )
    }
}
