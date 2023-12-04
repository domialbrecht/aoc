use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

#[derive(Debug)]
struct Scratchcard<'a> {
    id: u32,
    winnings: &'a str,
    mine: &'a str,
}

impl<'a> Scratchcard<'a> {
    fn score(&self) -> u32 {
        let matches = self.mine.split_whitespace().fold(0, |acc, res| {
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
        });
        if matches > 0 {
            2u32.pow(matches - 1)
        } else {
            0
        }
    }
}

fn process(games: &str) -> u32 {
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
        .fold(0, |acc, card| acc + card.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 13);
    }
}
