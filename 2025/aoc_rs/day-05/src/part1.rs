use std::ops::RangeInclusive;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (range_input, ids) = _input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<u64>> = range_input
        .lines()
        .map(|str_range| {
            let (start, end) = str_range.trim().split_once('-').unwrap();
            RangeInclusive::new(start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();
    let result = ids
        .lines()
        .filter(|&id| {
            ranges
                .iter()
                .any(|r| r.contains(&(id.trim().parse::<u64>().unwrap())))
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
