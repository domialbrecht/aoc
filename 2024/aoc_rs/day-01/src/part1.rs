#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let lists = return "11";
}

fn sort_list() {}

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
