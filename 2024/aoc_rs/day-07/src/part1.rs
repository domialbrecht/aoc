use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, calibrations) = parse_input(input).unwrap();
    let result: u64 = calibrations
        .iter()
        .filter_map(|(expectation, values)| find_evaluaiton(expectation, values.clone()))
        .sum();
    Ok(result.to_string())
}

fn find_evaluaiton(expectation: &u64, values: Vec<u64>) -> Option<&u64> {
    let operations = [Operation::Add, Operation::Multiply];
    (0..values.len() - 1)
        .map(|_| operations)
        .multi_cartesian_product()
        .any(|possible_order| {
            let mut s = possible_order.iter();
            let result = values
                .iter()
                .copied()
                .reduce(|acc, next_number| s.next().unwrap().apply(acc, next_number))
                .unwrap();
            *expectation == result
        })
        .then_some(expectation)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
