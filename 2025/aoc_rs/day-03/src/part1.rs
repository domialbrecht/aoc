use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let result = _input
        .lines()
        .map(|bank| {
            let bank = bank.trim();
            let (index, max_start) = &bank[..(bank.len() - 1)]
                .chars()
                .enumerate()
                .max_set_by_key(|(_i, battery)| *battery)
                .first()
                .cloned()
                .unwrap();
            let (_, max_end) = &bank[(index + 1)..]
                .chars()
                .enumerate()
                .max_by_key(|(_i, battery)| *battery)
                .unwrap();
            format!("{max_start}{max_end}").parse::<u64>().unwrap()
        })
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_99() -> miette::Result<()> {
        let input = "9987654321111111
818181911112199";
        assert_eq!("198", process(input)?);
        Ok(())
    }
}
