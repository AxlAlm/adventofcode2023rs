use core::panic;
use std::{char, collections::HashMap};

const FIVE_OF_A_KIND_SCORE: u32 = 7;
const FOUR_OF_A_KIND_SCORE: u32 = 6;
const FULL_HOUSE_SCORE: u32 = 5;
const THREE_OF_A_KIND_SCORE: u32 = 4;
const TWO_PAIR_SCORE: u32 = 3;
const ONE_PAIR_SCORE: u32 = 2;
const HIGH_CARD_SCORE: u32 = 1;

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    cards_score: u32,
    bid: u32,
}

fn card_to_value(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 0,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}

fn card_counts_to_score(card_counts: &HashMap<char, u32>) -> u32 {
    let mut card_counts: Vec<String> = card_counts
        .values()
        .into_iter()
        .map(|x| format!("{count}", count = x))
        .collect();
    card_counts.sort();
    card_counts.reverse();
    let card_counts_str = card_counts.join("");
    match card_counts_str.as_str() {
        "5" => FIVE_OF_A_KIND_SCORE,
        "41" => FOUR_OF_A_KIND_SCORE,
        "32" => FULL_HOUSE_SCORE,
        "311" => THREE_OF_A_KIND_SCORE,
        "221" => TWO_PAIR_SCORE,
        "2111" => ONE_PAIR_SCORE,
        "11111" => HIGH_CARD_SCORE,
        _ => 0,
    }
}

fn calc_cards_score(cards: &Vec<char>) -> u32 {
    let mut card_counts: HashMap<char, u32> = HashMap::new();
    for c in cards.iter() {
        *card_counts.entry(c.to_owned()).or_insert(0) += 1;
    }
    let mut best_score = card_counts_to_score(&card_counts);
    if card_counts.contains_key(&'J') {
        for c in card_counts.keys() {
            if c == &'J' {
                continue;
            }
            let mut card_counts_copy = card_counts.clone();
            let j = card_counts_copy.remove(&'J').unwrap();
            *card_counts_copy.get_mut(c).unwrap() += j;

            best_score = std::cmp::max(best_score, card_counts_to_score(&card_counts_copy));
        }
    }

    return best_score;
}

impl Hand {
    fn compare(&self, hand: &Hand) -> std::cmp::Ordering {
        if self.cards_score > hand.cards_score {
            return std::cmp::Ordering::Greater;
        }

        if self.cards_score < hand.cards_score {
            return std::cmp::Ordering::Less;
        }

        for i in 0..5 {
            if card_to_value(self.cards[i]) > card_to_value(hand.cards[i]) {
                return std::cmp::Ordering::Greater;
            }

            if card_to_value(self.cards[i]) < card_to_value(hand.cards[i]) {
                return std::cmp::Ordering::Less;
            }
        }

        panic!("Failed to compare! Should not get here");
        // return std::cmp::Ordering::Equal;
    }
}

fn parse_game(input: &str) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let cards_bid: Vec<&str> = line.split(" ").collect();
        let cards: Vec<char> = cards_bid[0].chars().collect();
        let cards_score = calc_cards_score(&cards);
        hands.push(Hand {
            cards: cards,
            cards_score: cards_score,
            bid: cards_bid[1].parse().unwrap_or(0),
        })
    }
    return hands;
}

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut hands = parse_game(input);
    hands.sort_by(|a, b| a.compare(b));
    let mut total_winnings = 0;
    for (i, h) in hands.iter().enumerate() {
        total_winnings += h.bid * (i as u32 + 1);
    }
    return total_winnings;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {

        let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let expected_result = 5905;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}

