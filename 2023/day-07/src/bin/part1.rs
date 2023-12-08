use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-06/input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> usize {
    let data = get_data(games);
    todo!()
}

fn get_data(games: &str) -> Vec<(u32, u32)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 288);
    }
}
