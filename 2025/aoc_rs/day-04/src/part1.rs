use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug)]
struct GridPos {
    x: i32,
    y: i32,
}

const LOCATIONS: [GridPos; 8] = [
    GridPos { x: 0, y: 1 },
    GridPos { x: 1, y: 1 },
    GridPos { x: 1, y: 0 },
    GridPos { x: 1, y: -1 },
    GridPos { x: 0, y: -1 },
    GridPos { x: -1, y: -1 },
    GridPos { x: -1, y: 0 },
    GridPos { x: -1, y: 1 },
];

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let grid_positions = _input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                (char == '@').then_some(GridPos {
                    x: x as i32,
                    y: y as i32,
                })
            })
        })
        .collect::<HashSet<GridPos>>();

    let count = grid_positions
        .iter()
        .filter(|&position| {
            LOCATIONS
                .iter()
                .filter(|&loc| {
                    grid_positions.contains(&GridPos {
                        x: position.x + loc.x,
                        y: position.y + loc.y,
                    })
                })
                .count()
                < 4
        })
        .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
