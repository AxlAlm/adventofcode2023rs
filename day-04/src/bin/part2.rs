use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Card {
    won_cards: Vec<u32>,
}

fn create_card_map(input: &str) -> HashMap<u32, Card> {
    let mut cards: HashMap<u32, Card> = HashMap::new();
    for line in input.trim().lines() {
        let cardheader_numbers: Vec<&str> = line.split(":").collect();
        let cardheader: Vec<&str> = cardheader_numbers[0].trim().split_whitespace().collect();
        let card_id = cardheader[1].parse::<u32>().unwrap();
        let numbers_sections: Vec<&str> = cardheader_numbers[1].trim().split("|").collect();
        let winning_numbers: Vec<u32> = numbers_sections[0]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let scraped_numbers: Vec<u32> = numbers_sections[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let winning_numbers_set: HashSet<u32> = HashSet::from_iter(winning_numbers);
        let scraped_numbers_set: HashSet<u32> = HashSet::from_iter(scraped_numbers);

        let mut won_cards = vec![];
        for (i, _) in winning_numbers_set
            .intersection(&scraped_numbers_set)
            .enumerate()
        {
            won_cards.push(card_id + (i as u32 + 1))
        }

        cards.insert(
            card_id,
            Card {
                won_cards: won_cards,
            },
        );
    }
    return cards;
}

fn find_won_cards(card_map: &HashMap<u32, Card>, won_cards: &Vec<u32>) -> u32 {
    let mut nr_won_cards = 0;
    for card_id in won_cards.iter() {
        nr_won_cards += 1;
        let won_card_ids = &card_map[card_id].won_cards;
        nr_won_cards += find_won_cards(card_map, won_card_ids);
    }
    return nr_won_cards;
}

fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let lenght_lines = lines.len();
    let card_map = create_card_map(input);

    let mut nr_cards_won = card_map.len() as u32;
    for i in 1..lenght_lines + 1 {
        let won_card_ids = &card_map[&(i as u32)].won_cards;
        nr_cards_won += find_won_cards(&card_map, won_card_ids);
    }

    return nr_cards_won;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {

        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            ";
        let expected_result = 30;
        let result = part2(test_input);
        assert_eq!(expected_result, result);
    }
}

