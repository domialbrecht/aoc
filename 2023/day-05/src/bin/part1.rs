use std::fs;

fn main() {
    let contents = fs::read_to_string("input-test.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> u32 {
    let mut l_iter = games.lines();
    let seeds = l_iter.next();
    dbg!(games.lines().collect::<Vec<&str>>());
    todo!()
}

fn get_mapping(input: u32) -> u32 {
    todo!()
}

fn seed_to_soil(seed: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soil() {
        assert_eq!(seed_to_soil(79), 81);
        assert_eq!(seed_to_soil(14), 14);
        assert_eq!(seed_to_soil(55), 57);
        assert_eq!(seed_to_soil(13), 13);
    }

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 35);
    }
}
