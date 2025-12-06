use rangemap::RangeInclusiveSet;
use std::ops::RangeInclusive;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (range_input, _) = _input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<u64>> = range_input
        .lines()
        .map(|str_range| {
            let (start, end) = str_range.trim().split_once('-').unwrap();
            RangeInclusive::new(start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();

    let mut range_set = RangeInclusiveSet::new();
    for range in ranges {
        range_set.insert(range);
    }

    let result = range_set
        .iter()
        .map(|range| range.end() + 1 - range.start())
        .sum::<u64>();

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
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
