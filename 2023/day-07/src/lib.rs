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
    fn get_type(mut card_counts: HashMap<char, usize>) -> HandType {
        if let Some(&jcount) = card_counts.get(&'J') {
            if card_counts.len() > 1 {
                card_counts.remove(&'J');
                if let Some((&best_key, _)) = card_counts
                    .iter()
                    .max_set_by_key(|&(_, &value)| (value))
                    .iter()
                    .max_by_key(|&(key, _)| card_score(**key))
                {
                    card_counts.entry(best_key).and_modify(|e| *e += jcount);
                }
            }
        }

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

pub fn card_score(char: char) -> u32 {
    match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        value => value.to_digit(10).unwrap(),
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
        let card_score = self.cards.chars().map(card_score).collect_tuple().unwrap();
        (self.handtype, card_score)
    }
}
