use glam::I64Vec2;

use itertools::Itertools;

use crate::custom_error::AocError;
use tracing::{info, span, Level};

#[tracing::instrument(skip(content))]
pub fn process(content: &str, expansion_size: i64) -> miette::Result<String, AocError> {
    let empty_rows = content
        .lines()
        .enumerate()
        .filter_map(|(index, line)| line.chars().all(|c| c == '.').then_some(index))
        .collect::<Vec<usize>>();

    let mut columns = content.lines().map(|line| line.chars()).collect::<Vec<_>>();
    let empty_columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .enumerate()
    .filter_map(|(index, column)| column.iter().all(|c| c == &'.').then_some(index))
    .collect::<Vec<usize>>();

    let galaxies = content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '#' => Some(I64Vec2::new(x as i64, y as i64)),
                    _ => None,
                })
        })
        .collect::<Vec<I64Vec2>>();

    let res = galaxies
        .iter()
        .combinations(2)
        .map(|s| {
            let galaxy_a_id = galaxies.iter().position(|v| v == s[0]).unwrap() + 1;
            let galaxy_b_id = galaxies.iter().position(|v| v == s[1]).unwrap() + 1;
            let my_span = span!(
                Level::INFO,
                "galaxy_combi_span",
                ids = format!(
                    "{}-{}",
                    galaxy_a_id.min(galaxy_b_id),
                    galaxy_a_id.max(galaxy_b_id)
                ),
                galaxy_a_id,
                galaxy_b_id,
                galaxy_a = ?galaxies
                    .iter()
                    .find(|v| v == &s[0])
                    .unwrap(),
                galaxy_b = ?galaxies
                    .iter()
                    .find(|v| v == &s[1])
                    .unwrap() // duplicates = acc.get(&index)
            );
            my_span.in_scope(|| {
                distance(
                    expanded_galaxy(s[0], &empty_rows, &empty_columns, expansion_size),
                    expanded_galaxy(s[1], &empty_rows, &empty_columns, expansion_size),
                )
            })
        })
        .inspect(|d| info!(?d))
        .sum::<i64>();
    Ok(res.to_string())
}

fn expanded_galaxy(
    galaxy: &I64Vec2,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_size: i64,
) -> I64Vec2 {
    let expand_rows = empty_rows
        .iter()
        .position(|row| row > &(galaxy.y as usize))
        .unwrap_or(empty_rows.len());
    let expand_cols = empty_cols
        .iter()
        .position(|col| col > &(galaxy.x as usize))
        .unwrap_or(empty_cols.len());
    info!(expand_rows, expand_cols);
    *galaxy
        + I64Vec2::new(
            expand_cols as i64 * (expansion_size - 1),
            expand_rows as i64 * (expansion_size - 1),
        )
}

fn distance(first: I64Vec2, second: I64Vec2) -> i64 {
    info!(?first);
    info!(?second);
    let v = (first - second).abs();
    (v.x + v.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 1030)]
    #[case(100, 8410)]
    #[test_log::test]
    fn test_process(#[case] expansion_size: i64, #[case] expected: i64) -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(expected.to_string(), process(input, expansion_size)?);
        Ok(())
    }
}
