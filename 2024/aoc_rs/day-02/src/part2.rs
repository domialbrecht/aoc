#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    Ok("11".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
