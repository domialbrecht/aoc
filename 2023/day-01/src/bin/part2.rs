use std::fs;

fn part_02(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");

            println!("{}", line);
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|numbers| numbers[0] * 10 + numbers[numbers.len() - 1])
        .sum()
}

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("Should read file");
    let total = part_02(&contents);
    println!("total {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let input = String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\none1one\nontwaonx");
        assert_eq!(part_02(&input), 292);
    }

    #[test]
    fn test_part_02_single() {
        let input = String::from("512ninexrqpvktwoner");
        assert_eq!(part_02(&input), 51);
    }
}
