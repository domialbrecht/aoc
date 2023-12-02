use std::{collections::HashMap, fs};

const DICES: [i32; 3] = [12, 13, 14];

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> u32 {
    let lines = games.lines();
    let mut total = 0;
    for line in lines {
        total += get_game_score(line);
    }
    total
}

fn get_game_score(game: &str) -> u32 {
    let (info, colors) = game.split_once(':').unwrap();
    let id = get_id(info);

    for pull in colors.split(';') {
        let colors = game_colors(pull);
        if !check_game(colors) {
            return 0;
        }
    }
    id
}

fn get_id(data: &str) -> u32 {
    let a: String = data.chars().filter(|c| c.is_ascii_digit()).collect();
    a.parse().unwrap()
}

fn game_colors(pull: &str) -> HashMap<&str, i32> {
    let mut colors: HashMap<&str, i32> = HashMap::new();
    for pair in pull.split(',') {
        let pair = pair.trim();
        let parts: Vec<&str> = pair.split_whitespace().collect();
        if let Some(count) = parts.first().and_then(|c| c.parse::<i32>().ok()) {
            // Update the count in the HashMap
            *colors.entry(parts[1]).or_insert(0) += count;
        }
    }
    colors
}

fn check_game(game: HashMap<&str, i32>) -> bool {
    let colors = ["red", "green", "blue"];
    for (i, amount) in DICES.iter().enumerate() {
        let valid = match game.get(colors[i]) {
            Some(color) => color <= amount,
            None => true,
        };

        if !valid {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
Game 90: 9 red, 12 green, 2 blue; 1 blue, 11 green, 10 red; 10 red, 2 green
";
        assert_eq!(process(input), 98);
    }

    #[test]
    fn test_sum() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
Game 98: 9 red, 12 green, 2 blue; 1 blue, 11 green, 10 red; 10 red, 2 green
Game 99: 4 red, 13 blue, 7 green; 7 green, 5 blue, 6 red; 7 green, 11 blue; 10 green, 2 red, 8 blue
Game 100: 2 green, 1 blue; 9 red, 8 green, 1 blue; 4 red, 10 green, 1 blue; 17 green, 8 red; 5 green, 1 blue, 7 red; 14 red, 12 green
";
        let asserts = [true, true, false, false, true, true, true, false];
        for (i, line) in input.lines().enumerate() {
            assert_eq!(get_game_score(line) > 0, asserts[i]);
        }
    }
}
