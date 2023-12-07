use std::{cmp::Ordering, collections::HashMap};

use crate::parse_file_input;

pub fn run() {
    let input = parse_file_input(7);
    println!("Running Day 7");
    println!(
        "Part 1 :: What are the total winnings? --> {:?}",
        part1(&input)
    );
    println!(
        "Part 2 :: What are the new total winnings? --> {:?}",
        part2(&input)
    );
}

fn part1(input: &str) -> usize {
    let mut rounds = input_to_rounds(input, false);
    rounds.sort_by(|a, b| a.hand.cmp(&b.hand));
    calculate_winnings(rounds)
}

fn part2(input: &str) -> usize {
    let mut rounds = input_to_rounds(input, true);
    rounds.sort_by(|a, b| a.hand.cmp(&b.hand));
    calculate_winnings(rounds)
}

#[derive(Debug)]
struct Round {
    hand: Hand,
    bid: usize,
}

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    with_jokers: bool,
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards.iter().all(|c| other.cards.contains(c))
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
const CARD_VALUES_ORDERED_NO_JOKERS: &str = "AKQJT98765432";
const CARD_VALUES_ORDERED_WITH_JOKERS: &str = "AKQT98765432J";
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_values = match self.with_jokers {
            true => CARD_VALUES_ORDERED_WITH_JOKERS,
            false => CARD_VALUES_ORDERED_NO_JOKERS,
        };
        if self.hand_type != other.hand_type {
            // HandType is ordered by hand strength, so simply compare
            self.hand_type.cmp(&other.hand_type)
        } else {
            // The HandTypes are the same, compare card-by-card
            for i in 0..self.cards.len() {
                if self.cards[i] == other.cards[i] {
                    continue;
                } else if let Some(self_card_val) = card_values.find(self.cards[i]) {
                    if let Some(other_card_val) = card_values.find(other.cards[i]) {
                        return usize::cmp(&self_card_val, &other_card_val);
                    } else {
                        unreachable!("Unexpected card {:?}", other.cards[i]);
                    }
                } else {
                    unreachable!("Unexpected card {:?}", self.cards[i]);
                }
            }
            Ordering::Equal
        }
    }
}

impl Hand {
    fn new(cards: &str, with_jokers: bool) -> Hand {
        let cards = cards[0..5]
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        Hand {
            cards,
            hand_type: HandType::new(cards, with_jokers),
            with_jokers,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn new(cards: [char; 5], with_jokers: bool) -> HandType {
        let mut card_occurrences = HashMap::new();
        for card in cards {
            let cnt = card_occurrences.entry(card).or_insert(0);
            *cnt += 1;
        }
        let mut num_jokers = 0;
        // Extract the jokers, unless it's FiveOfAKind (either no jokers or all jokers - result is FiveOfAKind either way)
        if with_jokers && card_occurrences.len() > 1 {
            num_jokers = card_occurrences.remove(&'J').unwrap_or(0);
        }
        let mut card_occurrences = card_occurrences.into_iter().collect::<Vec<(char, u32)>>();
        card_occurrences.sort_by(|a, b| b.1.cmp(&a.1));
        // Add the jokers back in, treating them as wildcards to the most frequent occurrence
        if num_jokers > 0 {
            card_occurrences[0].1 += num_jokers;
        }

        match card_occurrences[0].1 {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match card_occurrences[1].1 {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match card_occurrences[1].1 {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        }
    }
}

fn input_to_rounds(input: &str, with_jokers: bool) -> Vec<Round> {
    input
        .lines()
        .map(|line| Round {
            hand: Hand::new(&line[0..5], with_jokers),
            bid: line[6..]
                .parse()
                .unwrap_or_else(|e| panic!("Unable to parse bid from line '{line}': {e}")),
        })
        .collect()
}

fn calculate_winnings(sorted_rounds: Vec<Round>) -> usize {
    let mut total_winnings = 0;

    for (i, round) in sorted_rounds.iter().rev().enumerate() {
        total_winnings += round.bid * (i + 1);
    }

    total_winnings
}

#[cfg(test)]
mod tests {
    use crate::day07::{part1, part2};

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(6440, part1(SAMPLE_INPUT))
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(5905, part2(SAMPLE_INPUT));
    }
}
