use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> i32 {
    let lines = games.lines();
    let mut total = 0;
    for game in lines {
        total += get_game_score(game);
    }
    total
}

fn get_game_score(game: &str) -> i32 {
    let (_, colors) = game.split_once(':').unwrap();

    let mut min_colors: HashMap<&str, i32> = HashMap::new();
    for pull in colors.split(';') {
        let colors = game_colors(pull);
        for (color, amount) in colors.iter() {
            if let Some(min) = min_colors.get_mut(color) {
                if amount > min {
                    *min = *amount;
                }
            } else {
                min_colors.insert(color, *amount);
            }
        }
    }
    min_colors.values().product()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&input), 2286);
    }

    #[test]
    fn test_power() {
        let input = fs::read_to_string("input-test.txt").expect("Should read file");
        let asserts = [48, 12, 1560, 630, 36];
        for (i, line) in input.lines().enumerate() {
            assert_eq!(get_game_score(line), asserts[i]);
        }
    }
}
