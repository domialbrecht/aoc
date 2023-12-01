use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    process(&contents);
}

fn process(input: &str) -> u32 {
    let lines: Vec<String> = input
        .lines()
        .map(String::from)
        .filter(|x| !x.is_empty())
        .collect();
    println!("{:?}", lines);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
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
        assert_eq!(process(input), 24000)
    }
}
