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
        .map(|n| match repeating_string(n.to_string()) {
            true => n,
            false => 0,
        })
        .sum()
}

//https://leetcode.com/problems/repeated-substring-pattern/description/
fn repeating_string(num: String) -> bool {
    let nums = num.as_str();
    let doubled = format!("{nums}{nums}");
    doubled[1..doubled.len() - 1].contains(nums)
}

fn repeating_string_iter(input: String) -> bool {
    let characters: Vec<char> = input.chars().collect();
    let total_len = characters.len();

    (1..=total_len / 2)
        .filter(|&pattern_len| total_len.is_multiple_of(pattern_len))
        .any(|pattern_len| {
            characters
                .iter()
                .enumerate()
                .skip(pattern_len)
                .all(|(idx, &ch)| ch == characters[idx % pattern_len])
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }

    #[test]
    fn test_repeat_111_v() -> miette::Result<()> {
        let input = "111";
        assert!(repeating_string_iter(input.to_string()));
        Ok(())
    }

    #[test]
    fn test_repeat_1188511880_v() -> miette::Result<()> {
        let input = "1188511880";
        assert!(!repeating_string_iter(input.to_string()));
        Ok(())
    }

    #[test]
    fn test_repeat_1188511885_v() -> miette::Result<()> {
        let input = "1188511885";
        assert!(repeating_string_iter(input.to_string()));
        Ok(())
    }

    #[test]
    fn test_repeat_111() -> miette::Result<()> {
        let input = "111";
        assert!(repeating_string(input.to_string()));
        Ok(())
    }

    #[test]
    fn test_repeat_1188511880() -> miette::Result<()> {
        let input = "1188511880";
        assert!(!repeating_string(input.to_string()));
        Ok(())
    }

    #[test]
    fn test_repeat_1188511885() -> miette::Result<()> {
        let input = "1188511885";
        assert!(repeating_string(input.to_string()));
        Ok(())
    }
}
