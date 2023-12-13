use std::fs;

use itertools::{Itertools, Position};

fn main() {
    let contents = fs::read_to_string("./day-09/input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

// 0   3   6   9  12  15
//   3   3   3   3   3
//     0   0   0   0
fn process(content: &str) -> i64 {
    content
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut first_nums: Vec<i64> = vec![];
            loop {
                if nums.iter().all(|num| num == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(position, (left, right))| {
                        match position {
                            Position::First | Position::Only => first_nums.push(*left),
                            _ => {}
                        }
                        right - left
                    })
                    .collect::<Vec<i64>>()
            }
            let result = first_nums.iter().rev().fold(0, |acc, num| num - acc);
            result
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 2);
    }
}
