use std::collections::HashMap;

/// Day 7: Camel Cards, Part 1

const FIVE_OF_A_KIND: u32 = 7;
const FOUR_OF_A_KIND: u32 = 6;
const FULL_HOUSE: u32 = 5;
const THREE_OF_A_KIND: u32 = 4;
const TWO_PAIR: u32 = 3;
const ONE_PAIR: u32 = 2;
const HIGH_CARD: u32 = 1;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    kind: u32,
    values: Vec<u32>,
    bid: u32,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", get_winnings(input));
}

fn get_winnings(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(|line| parse_line(line)).collect();
    hands.sort_by(|a, b| (a.kind, &a.values).cmp(&(b.kind, &b.values)));
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum()
}

fn parse_line(line: &str) -> Hand {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let mut hand: Hand = parse_hand(parts[0]);
    let bid: u32 = parts[1].parse().unwrap();
    hand.bid = bid;
    hand
}

fn parse_hand(hand: &str) -> Hand {
    let mut values: Vec<u32> = Vec::new();
    let mut cards: HashMap<char, u32> = HashMap::new();
    for card in hand.chars() {
        for (i, card_label) in CARDS.iter().enumerate() {
            if card == *card_label {
                values.push(i as u32);
            }
        }
        cards.insert(card, hand.chars().filter(|c| *c == card).count() as u32);
    }
    let counts: Vec<&u32> = cards.values().collect();
    let kind = if counts.len() == 5 {
        HIGH_CARD
    } else if counts.contains(&&4) {
        FOUR_OF_A_KIND
    } else if counts.contains(&&3) && counts.contains(&&2) {
        FULL_HOUSE
    } else if counts.contains(&&3) {
        THREE_OF_A_KIND
    } else if counts.contains(&&5) {
        FIVE_OF_A_KIND
    } else if counts.contains(&&2) && counts.len() == 3 {
        TWO_PAIR
    } else {
        ONE_PAIR
    };
    Hand {
        kind,
        values,
        bid: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_get_winnings() {
        assert_eq!(get_winnings(TEST_INPUT), 6440);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse_hand("22TTT"),
            Hand {
                kind: FULL_HOUSE,
                values: vec![0, 0, 8, 8, 8],
                bid: 0
            }
        );
        assert_eq!(
            parse_hand("33333"),
            Hand {
                kind: FIVE_OF_A_KIND,
                values: vec![1, 1, 1, 1, 1],
                bid: 0
            }
        );
        assert_eq!(
            parse_hand("2AAAA"),
            Hand {
                kind: FOUR_OF_A_KIND,
                values: vec![0, 12, 12, 12, 12],
                bid: 0
            }
        );
    }
}
