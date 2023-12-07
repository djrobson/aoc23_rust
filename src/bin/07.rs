aoc23_rust::solution!(7);
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}
impl Card {
    // ???
    fn new(card: &char) -> Card {
        match card {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("unexpected card type"),
        }
    }
    fn new2(card: &char) -> Card {
        match card {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("unexpected card type"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // Implement equality logic here
        // Compare all fields for equality
        self.cards == other.cards && self.hand_type == other.hand_type && self.bid == other.bid
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Implement comparison logic here
        // For example, compare the hand_type and bid fields
        // Return Ordering::Less, Ordering::Equal, or Ordering::Greater
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // we're the same hand type, scan for highest card
                for n in 0..self.cards.len() {
                    match self.cards[n].cmp(&other.cards[n]) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                    }
                }
                // we made it all the way through
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_hand_type1(hand: &[Card]) -> HandType {
    let mut cards: HashMap<&Card, u32> = HashMap::new();

    for card in hand.iter() {
        let entry = cards.entry(card).or_insert(0);
        *entry += 1;
    }

    let key_count = cards.keys().len();
    let values = cards.values().collect::<Vec<&u32>>();
    if key_count == 1 {
        HandType::Five
    } else if key_count == 2 {
        // check for 4ook and full house
        if values[0] == &4 || values[1] == &4 {
            HandType::Four
        } else {
            HandType::FullHouse
        }
    } else if key_count == 3 {
        if values[0] == &3 || values[1] == &3 || values[2] == &3 {
            HandType::Three
        } else {
            HandType::TwoPair
        }
    } else if key_count == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn get_hand_type2(hand: &[Card]) -> HandType {
    let mut cards: HashMap<&Card, u32> = HashMap::new();

    for card in hand.iter() {
        let entry = cards.entry(card).or_insert(0);
        *entry += 1;
    }

    let key_count = cards.keys().len();
    let values = cards.values().collect::<Vec<&u32>>();
    if key_count == 1 {
        HandType::Five
    } else if key_count == 2 {
        // check for 4ook and full house
        if values[0] == &4 || values[1] == &4 {
            todo!("check for joker");
            HandType::Four
        } else {
            todo!("check for joker");
            HandType::FullHouse
        }
    } else if key_count == 3 {
        if values[0] == &3 || values[1] == &3 || values[2] == &3 {
            todo!("check for joker");
            HandType::Three
        } else {
            todo!("check for joker");
            HandType::TwoPair
        }
    } else if key_count == 4 {
        todo!("check for joker");
        HandType::OnePair
    } else {
        todo!("check for joker");
        HandType::HighCard
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let cards = &line[..5]
                .chars()
                .map(|card| Card::new(&card))
                .collect::<Vec<Card>>();
            Hand {
                cards: cards.clone(),
                bid: line[6..].parse().unwrap(),
                hand_type: get_hand_type1(cards),
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort();

    //dbg!("{}", &hands);

    let score = hands
        .iter()
        .enumerate()
        .map(|(offset, hand)| (offset as u32 + 1) * hand.bid)
        .sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let cards = &line[..5]
                .chars()
                .map(|card| Card::new2(&card))
                .collect::<Vec<Card>>();
            Hand {
                cards: cards.clone(),
                bid: line[6..].parse().unwrap(),
                hand_type: get_hand_type2(cards),
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort();

    let score = hands
        .iter()
        .enumerate()
        .map(|(offset, hand)| (offset as u32 + 1) * hand.bid)
        .sum();
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc23_rust::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
