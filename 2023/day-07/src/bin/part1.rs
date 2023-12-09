use std::fs;

use day07::{Hand, HandScore};
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("./day-07/input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> u32 {
    get_data(games)
        .iter()
        .sorted_by_key(|x| (x.0 .0 as u8, x.0 .1))
        .enumerate()
        .map(|(index, (_hand, bid))| (index as u32 + 1) * bid)
        .sum::<u32>()
}

fn get_data(games: &str) -> Vec<(HandScore, u32)> {
    games
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').expect("split hand and bet");
            (
                Hand::create(hand).hand_score(),
                bet.parse::<u32>().expect("parse bet as u32"),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 6440);
    }
}
