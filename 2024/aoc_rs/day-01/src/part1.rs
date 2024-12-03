#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = _input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(r, l)| {
                (
                    r.trim().parse::<usize>().unwrap(),
                    l.trim().parse::<usize>().unwrap(),
                )
            })
        })
        .unzip();

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
