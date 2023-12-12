use core::panic;
use std::{char, collections::HashMap};

// const CARD_VALUES: &str = "AKQJT98765432";

const FIVE_OF_A_KIND_RANK: u32 = 1;
const FOUR_OF_A_KIND_RANK: u32 = 2;
const FULL_HOUSE_RANK: u32 = 3;
const THREE_OF_A_KIND_RANK: u32 = 4;
const TWO_PAIR_RANK: u32 = 5;
const ONE_PAIR_RANK: u32 = 6;
const HIGH_CARD_RANK: u32 = 7;

// enum HandTypes {
//     FiveOfAKind,
//     FourOfAkind,
//     FullHouse,

// }
#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

fn card_to_value(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
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

fn cards_to_value(card_counts: &str) -> u32 {
    match card_counts {
        "5" => FIVE_OF_A_KIND_RANK,
        "41" => FOUR_OF_A_KIND_RANK,
        "32" => FULL_HOUSE_RANK,
        "311" => THREE_OF_A_KIND_RANK,
        "221" => TWO_PAIR_RANK,
        "2111" => ONE_PAIR_RANK,
        "11111" => HIGH_CARD_RANK,
        _ => 0,
    }
}

impl Hand {
    fn get_type(&self) -> u32 {
        let mut card_counts: HashMap<char, u32> = HashMap::new();
        for c in self.cards.iter() {
            *card_counts.entry(c.to_owned()).or_insert(0) += 1;
        }
        let mut card_counts: Vec<String> = card_counts
            .values()
            .into_iter()
            .map(|x| format!("{count}", count = x))
            .collect();
        card_counts.sort();
        card_counts.reverse();
        let card_counts_str = card_counts.join("");

        // dbg!(&self.cards);
        // dbg!(&card_counts_str);
        // dbg!(cards_to_value(&card_counts_str));
        return cards_to_value(&card_counts_str);
    }

    fn compare(&self, hand: &Hand) -> std::cmp::Ordering {
        if self.get_type() > hand.get_type() {
            return std::cmp::Ordering::Greater;
        }

        if self.get_type() < hand.get_type() {
            return std::cmp::Ordering::Less;
        }

        for i in 0..5 {
            // dbg!(self.cards[i], hand.cards[i]);
            // dbg!(card_to_value(self.cards[i]), card_to_value(hand.cards[i]));
            // dbg!(card_to_value(self.cards[i]) > card_to_value(hand.cards[i]));
            // dbg!(card_to_value(self.cards[i]) < card_to_value(hand.cards[i]));

            if card_to_value(self.cards[i]) > card_to_value(hand.cards[i]) {
                return std::cmp::Ordering::Less;
            }

            if card_to_value(self.cards[i]) < card_to_value(hand.cards[i]) {
                return std::cmp::Ordering::Greater;
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
        hands.push(Hand {
            cards: cards_bid[0].chars().collect(),
            bid: cards_bid[1].parse().unwrap_or(0),
        })
    }
    return hands;
}

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut hands = parse_game(input);
    hands.sort_by(|a, b| a.compare(b));
    hands.reverse();
    // let x: Vec<u32> = hands.iter().map(|x| x.get_type()).collect();
    // dbg!(&x);
    // dbg!(&hands);

    // dbg!(&hands[hands.len() - 1], &hands[hands.len() - 2]);
    // dbg!(hands[hands.len() - 1].compare(&hands[hands.len() - 2]));

    // dbg!(&hands[0], &hands[1]);
    // dbg!(hands[0].compare(&hands[1]));

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
    fn test_part_1() {

        let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let expected_result = 6440;
        let result = part1(test_input);
        assert_eq!(expected_result, result);
    }
}

