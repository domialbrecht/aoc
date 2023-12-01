use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("Should read file");

    let contents = contents.lines();

    let mut total: u32 = 0;
    for line in contents {
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
        let combined: u32 = combined.parse().expect("string");
        total += combined;
    }
    println!("totel {total}");
}
