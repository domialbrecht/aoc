use std::{collections::BTreeMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let contents = fs::read_to_string("./day-08/input.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parser(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;

    // skip empty
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

#[tracing::instrument]
fn process(contents: &str) -> usize {
    let (_, (instructions, map)) = parser(contents).expect("should validly parse");
    let mut node = "AAA";
    let Some(step_count) =
        instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(index, instruction)| {
                let options = map.get(node).expect("always have valid node");
                let next_node = match instruction {
                    Direction::Left => options.0,
                    Direction::Right => options.1,
                };
                if next_node == "ZZZ" {
                    Some(index + 1)
                } else {
                    node = next_node;
                    None
                }
            })
    else {
        panic!("infinite error")
    };
    step_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let contents = fs::read_to_string("input-test.txt").expect("Should read file");
        assert_eq!(process(&contents), 2);
    }

    #[test]
    fn test_game_2() {
        let contents = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(process(contents), 6);
    }
}
