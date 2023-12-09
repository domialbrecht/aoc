use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

impl HandType {
    // 55555 1
    // 55551 2
    // 55533 2
    // 55521 3
    // 55331 3
    // 11432 4
    // 12345 5
    fn get_type(card_counts: HashMap<char, usize>) -> HandType {
        if card_counts.len() == 1 {
            HandType::FiveKind
        } else if card_counts.len() == 2 && card_counts.values().any(|x| *x == 4) {
            HandType::FourKind
        } else if card_counts.len() == 2 {
            HandType::FullHouse
        } else if card_counts.len() == 3 && card_counts.values().any(|x| *x == 3) {
            HandType::ThreeKind
        } else if card_counts.len() == 3 {
            HandType::TwoPair
        } else if card_counts.len() == 4 {
            HandType::Pair
        } else if card_counts.len() == 5 {
            HandType::HighCard
        } else {
            panic!("Invalid hand type")
        }
    }
}

pub type HandScore = (HandType, (u32, u32, u32, u32, u32));

pub struct Hand<'a> {
    cards: &'a str,
    handtype: HandType,
}

impl<'a> Hand<'a> {
    pub fn create(cards: &str) -> Hand {
        if cards.len() != 5 {
            panic!("Invalid hand")
        }
        Hand {
            cards,
            handtype: HandType::get_type(cards.chars().counts_by(|char| char)),
        }
    }

    pub fn hand_score(&self) -> HandScore {
        let card_score = self
            .cards
            .chars()
            .map(|char| match char {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => char.to_digit(10).unwrap(),
            })
            .collect_tuple()
            .unwrap();
        (self.handtype, card_score)
    }
}
