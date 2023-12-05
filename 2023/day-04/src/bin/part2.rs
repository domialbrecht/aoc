use std::{collections::BTreeMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

#[derive(Clone)]
struct Scratchcard<'a> {
    id: u32,
    winnings: &'a str,
    mine: &'a str,
}

impl<'a> std::fmt::Debug for Scratchcard<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scratchcard").field("id", &self.id).finish()
    }
}

impl<'a> Scratchcard<'a> {
    fn matches(&self) -> usize {
        self.mine.split_whitespace().fold(0, |acc, res| {
            acc + if self
                .winnings
                .split_whitespace()
                .collect::<Vec<&str>>()
                .contains(&res)
            {
                1
            } else {
                0
            }
        })
    }
}

fn get_scratchcards(games: &str) -> Vec<Scratchcard> {
    games
        .lines()
        .map(|line| {
            let (game, scratches) = line
                .split_once(':')
                .expect("should be able to split away game");
            let (winning, mine) = scratches
                .split_once('|')
                .expect("should have winning and mine split");
            let game: String = game.chars().filter(|c| c.is_ascii_digit()).collect();
            Scratchcard {
                id: game.parse().unwrap(),
                winnings: winning.trim(),
                mine: mine.trim(),
            }
        })
        .collect()
}

fn process(games: &str) -> u32 {
    let cards = get_scratchcards(games);

    // How many of each card? At least one
    let store = (0..cards.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();

    // Data is the amount of new cards we generate.
    // 1 - 4
    // 2 - 2
    // 3 - 2
    // ...
    let data = cards.iter().map(|card| card.matches()).collect::<Vec<_>>();

    // We do this for x in data matches.
    // E.G 4 times for card one
    let result = data
        .iter()
        .enumerate()
        .fold(store, |mut acc, (index, card_score)| {
            // This is walking forwards on the copies. How many of the next card index do we need
            // to to add
            // Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
            // Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
            // Your copy of card 2 also wins one copy each of cards 3 and 4.
            let to_add = *acc.get(&index).unwrap();

            for i in (index + 1)..(index + 1 + *card_score) {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 30);
    }
}
