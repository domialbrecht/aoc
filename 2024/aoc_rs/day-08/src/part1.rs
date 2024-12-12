use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::{position, LocatedSpan};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse into hashmap with identifier as key, IVec as value
    // Iterate over keys, in pairs, check if tey resonate
    let (_input, mut antennas) =
        parse(Span::new(input)).map_err(|err| miette!("parse failed with {}", err))?;
    let x_size = input.lines().next().unwrap().len();
    let y_size = input.lines().count();
    let x_bounds = 0i32..x_size as i32;
    let y_bounds = 0i32..y_size as i32;

    antennas.sort_by(|a, b| a.1.cmp(&b.1));
    let result = antennas
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk.iter().combinations(2).flat_map(|stats| {
                let diff = stats[0].0 - stats[1].0;
                [stats[0].0 + diff, stats[1].0 - diff]
            })
        })
        .filter(|pos| x_bounds.contains(&pos.x) && y_bounds.contains(&pos.y))
        .unique()
        .count();

    Ok(result.to_string())
}

pub type Span<'a> = LocatedSpan<&'a str>;

fn pos_parse(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(take_till(|c: char| c.is_alphanum()), pos_parse))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
