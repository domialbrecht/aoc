use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let result = _input
        .lines()
        .map(|bank| {
            let bank = bank.trim();
            let mut batteries: Vec<char> = vec![];
            let mut current_index = 0;
            for i in 0..11 {
                let res = &bank[current_index..(bank.len() - 11 + i)]
                    .chars()
                    .enumerate()
                    .max_set_by_key(|(_i, battery)| *battery);
                let (index, max_start) = res.first().unwrap();

                batteries.push(*max_start);
                current_index += index + 1;
            }

            let (_, max_end) = &bank[(current_index)..]
                .chars()
                .enumerate()
                .max_by_key(|(_i, battery)| *battery)
                .unwrap();

            batteries.push(*max_end);

            batteries.iter().collect::<String>().parse::<u64>().unwrap()
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
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_9() -> miette::Result<()> {
        let input = "999999999999111";
        assert_eq!("999999999999", process(input)?);
        Ok(())
    }
}
