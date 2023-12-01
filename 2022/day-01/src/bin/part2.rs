use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("Total: {}", process(&contents));
}

fn process(input: &str) -> u32 {
    let mut leaderboard: Vec<u32> = vec![];
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let mut elf = 0;
    for line in lines {
        if line.is_empty() {
            update_leaderboard(&mut leaderboard, elf);
            elf = 0;
            continue;
        }
        elf += line.parse::<u32>().expect("number");
    }
    update_leaderboard(&mut leaderboard, elf);

    leaderboard.iter().sum()
}

fn update_leaderboard(leaderboard: &mut Vec<u32>, elf: u32) {
    println!("{}", elf);
    if leaderboard.len() < 3 {
        leaderboard.push(elf);
    } else if is_highscore(leaderboard, elf) {
        leaderboard.sort();
        leaderboard[0] = elf;
    }
}

fn is_highscore(leaderboard: &[u32], elf: u32) -> bool {
    leaderboard.iter().any(|&x| elf > x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part02() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(process(input), 45000)
    }
}
