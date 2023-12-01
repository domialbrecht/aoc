use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should read file");
    println!("{}", process(&contents));
}

fn process(input: &str) -> u32 {
    let mut max = 0;
    //walk lines, add up until empty
    // go to next line. reeat. if larger then before, replace
    // do until end of file
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let mut elf = 0;
    for line in lines {
        if line.is_empty() {
            if elf > max {
                max = elf;
            }
            elf = 0;
            continue;
        }
        elf += line.parse::<u32>().expect("number");
    }

    max
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
