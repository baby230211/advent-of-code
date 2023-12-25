use std::collections::HashMap;

fn main() {
    let contents = advent_of_code_2023::run().unwrap();
    let res = process(&contents).unwrap();
    println!("{}", res);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct HandScore {
    hand_type: HandType,
    cards: Vec<Card>,
}
impl PartialOrd for HandScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand_type.partial_cmp(&other.hand_type)
    }
}
impl HandScore {
    fn new(hand: &str) -> Self {
        let card_map = hand.chars().fold(HashMap::new(), |mut acc, c| {
            let count = acc.entry(c).or_insert(0);
            *count += 1;
            acc
        });
        let mut values = card_map.values().collect::<Vec<_>>();
        values.sort();
        let hand_type = match values.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::Pair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        };
        println!("{:?}", hand_type);
        let cards = hand
            .chars()
            .map(|c| match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => panic!("Invalid card"),
            })
            .collect::<Vec<_>>();
        HandScore { hand_type, cards }
    }
}
#[derive(Debug, Eq, PartialEq, Ord)]
struct Hand {
    bid: u64,
    score: HandScore,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score.hand_type == other.score.hand_type {
            return Some(self.score.cards.cmp(&other.score.cards));
        }
        Some(self.score.cmp(&other.score))
    }
}

fn process(contents: &str) -> Result<u64, ()> {
    let mut hands = contents
        .lines()
        .map(|line| {
            let (card_string, bid_amount) = line.split_once(" ").unwrap();
            Hand {
                bid: bid_amount.parse::<u64>().unwrap(),
                score: HandScore::new(card_string),
            }
        })
        .collect::<Vec<_>>();
    hands.sort();
    println!("{:?}", hands);
    let res = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum::<u64>();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(contents).unwrap());
    }
}
