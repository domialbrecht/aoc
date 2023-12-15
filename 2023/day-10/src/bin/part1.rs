use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-10/input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

fn process(content: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let contents = ".....
.S-7.
.|.|.
.L-J.
.....
";
        assert_eq!(process(&contents), 4);
    }

    #[test]
    fn test_two() {
        let contents = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(process(&contents), 8);
    }
}
