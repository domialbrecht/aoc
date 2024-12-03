use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let reports = _input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|c| c.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let result = reports
        .iter()
        .filter(|report| report.iter().tuple_windows().all(|(l, r)| l > r))
        .count();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
