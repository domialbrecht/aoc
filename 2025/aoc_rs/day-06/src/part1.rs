#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let sets: Vec<Vec<&str>> = _input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.trim()).collect())
        .collect();

    let (ops, nums) = sets.split_last().unwrap();
    dbg!(&nums);

    let result: u64 = ops
        .iter()
        .enumerate()
        .map(|(col_i, op)| {
            let num_it = (0..nums.len()).map(|row_i| nums[row_i][col_i].parse::<u64>().unwrap());

            match *op {
                "*" => num_it.product::<u64>(),
                "+" => num_it.sum(),
                _ => {
                    panic!("op match error")
                }
            }
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
