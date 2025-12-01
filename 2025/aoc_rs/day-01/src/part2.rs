use std::ops::Div;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, result) = _input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let dir = chars.next().unwrap();
            let amount: i32 = chars.as_str().trim().parse().unwrap();

            match dir {
                'L' => -amount,
                _ => amount,
            }
        })
        .fold((50i32, 0i32), |(dial, result), steps| {
            let next = dial + steps;

            let mut arounds = next.div(100).abs();
            if dial != 0 && next <= 0 {
                arounds += 1;
            }

            (next.rem_euclid(100), result + arounds)
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_half() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60";
        assert_eq!("3", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_big() -> miette::Result<()> {
        let input = "L1000";
        assert_eq!("10", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_right() -> miette::Result<()> {
        let input = "R65";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
