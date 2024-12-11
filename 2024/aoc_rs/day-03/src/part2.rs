use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let (_, result) =
        instructions
            .iter()
            .fold((ShouldDo::Do, 0), |(process, acc), ins| match ins {
                Instruction::Mul(a, b) => {
                    if process == ShouldDo::Do {
                        (process, acc + a * b)
                    } else {
                        (process, acc)
                    }
                }
                Instruction::Do => (ShouldDo::Do, acc),
                Instruction::Dont => (ShouldDo::Dont, acc),
            });
    Ok(result.to_string())
}

#[derive(PartialEq, Eq)]
enum ShouldDo {
    Do,
    Dont,
}

#[derive(Clone, Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

// Change this to mul_instruction
// Implement different instructions
// Return instruction type along instruction
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

// Use alt to match all instructions
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_rest, inst)| inst))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
