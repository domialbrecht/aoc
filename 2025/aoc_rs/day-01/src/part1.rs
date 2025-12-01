#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let instructions: Vec<i32> = _input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let dir = chars.next().unwrap();
            let amount: i32 = chars.as_str().trim().parse().unwrap();

            if let 'L' = dir {
                -amount
            } else {
                amount
            }
        })
        .collect();

    let mut pos = 50;
    let mut result = 0;
    for ins in instructions {
        let newpos = (pos + ins) % 100;
        pos = if newpos < 0 {
            100 - newpos.abs()
        } else {
            newpos.abs()
        };

        if pos == 0 || pos == 100 {
            result += 1;
        }
    }

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
        assert_eq!("3", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_over() -> miette::Result<()> {
        let input = "L50
R200";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
