use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("Should read file");

    println!("With text:\n{contents}");
}
