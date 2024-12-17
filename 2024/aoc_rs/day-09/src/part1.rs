#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let high_index: u32 = input.trim().chars().map(|c| c.to_digit(10).unwrap()).sum();
    let mut reverse = ((0..input.trim().len())
        .rev()
        .zip(input.trim().chars().rev()))
    .scan(high_index, |base_index, (compressed_i, c)| {
        let num_indic = c.to_digit(10).unwrap();
        *base_index -= num_indic;

        Some(
            (*base_index..(*base_index + num_indic))
                .rev()
                .filter_map(move |i| (compressed_i % 2 == 0).then_some((i, compressed_i / 2))),
        )
    })
    .flatten();

    let mut base_index = 0;
    let mut sum = 0;
    let mut lst_uncompressed = u32::MAX;

    for (compressed_i, c) in input.trim().chars().enumerate() {
        let num_indic = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_i / 2;

        for uncompressed_index in base_index..(base_index + num_indic) {
            if uncompressed_index >= lst_uncompressed as usize {
                break;
            }
            if compressed_i % 2 == 0 {
                sum += uncompressed_index * file_id;
            } else {
                let (rev_uncompressed_index, file_id) = reverse.next().unwrap();
                sum += uncompressed_index * file_id;
                lst_uncompressed = rev_uncompressed_index;
            }
        }

        base_index += num_indic;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1918", process(input)?);
        Ok(())
    }
}
