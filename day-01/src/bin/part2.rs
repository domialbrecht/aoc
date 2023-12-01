use std::fs;

fn part2(input: &str) -> u32 {
    let contents = input.lines();

    let mut total: u32 = 0;
    let wordchars = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in contents {
        // convert wordchars in line to numbers
        let line = line.to_string();
        println!("line: {}", line);

        // find position of wordchars in line and add to word wit its positon to a vector
        let mut v: Vec<(usize, &str)> = Vec::new();
        for word in wordchars.iter() {
            let mut pos = 0;
            while let Some(i) = line[pos..].find(word) {
                pos += i;
                v.push((pos, word));
                pos += word.len();
            }
        }

        // sort vector by position
        v.sort_by(|a, b| a.0.cmp(&b.0));

        //get the words for the vector
        let mut words: Vec<&str> = Vec::new();
        for (_, word) in v.iter() {
            words.push(word);
        }

        // replace the words with numbers
        let mut line = line;
        for (_, word) in words.iter().enumerate() {
            let num = wordchars.iter().position(|&r| r == *word).unwrap() + 1;
            line = line.replace(word, &num.to_string());
        }

        let v: Vec<&str> = line.matches(char::is_numeric).collect();
        let first = v.first();
        let mut last = v.last();
        if first.is_none() && last.is_none() {
            continue;
        }
        if last.is_none() {
            last = first;
        }
        let num1 = first.unwrap();
        let num2 = last.unwrap();

        let combined = format!("{num1}{num2}");
        println!("combined: {}", combined);
        assert!(combined.len() == 2);
        let combined: u32 = combined.parse().expect("string");
        total += combined;
    }
    total
}

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("Should read file");
    let total = part2(&contents);
    println!("total {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_02() {
        let input = String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        assert_eq!(part2(&input), 281);
    }
}
