use std::{fs, ops::Not};

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
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            std::iter::successors(Some(nums), |nums| {
                nums.iter().all(|num| num == &0).not().then_some(
                    nums.iter()
                        .tuple_windows::<(&i64, &i64)>()
                        .map(|(left, right)| right - left)
                        .collect(),
                )
            })
            .map(|v| *v.last().unwrap())
            .sum::<i64>()
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 114);
    }
}
