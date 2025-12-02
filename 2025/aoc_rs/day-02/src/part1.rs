#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let result: usize = _input
        .split_terminator(',')
        .map(|range| range.trim().split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(start, end)| get_range_sum(start, end))
        .sum();

    Ok(result.to_string())
}

fn get_range_sum(start: usize, end: usize) -> usize {
    (start..=end)
        .map(|n| match number_repeats(n.to_string()) {
            true => n,
            false => 0,
        })
        .sum()
}

fn number_repeats(num: String) -> bool {
    match num.len().checked_rem(2).is_none() {
        true => false,
        false => {
            let (a, b) = num.split_at(num.len() / 2);
            a == b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
