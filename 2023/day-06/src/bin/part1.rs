use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-06/input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(games: &str) -> usize {
    let data = get_data(games);
    data.iter().map(run_records).product()
}

fn run_records(run: &(u32, u32)) -> usize {
    let (max_time, distance) = *run;
    let wins = (0..max_time)
        .filter(|hold| {
            let time = max_time - hold;
            time * hold > distance
        })
        .count();
    println!("{wins}");
    wins
}

fn get_data(games: &str) -> Vec<(u32, u32)> {
    get_entries(games, 0)
        .zip(get_entries(games, 1))
        .skip(1)
        .inspect(|x| println!("{:?}", x))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>()
}

fn get_entries(games: &str, line: usize) -> std::str::SplitWhitespace<'_> {
    games.lines().nth(line).unwrap().split_whitespace()
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
