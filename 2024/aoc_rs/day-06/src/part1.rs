use std::collections::HashSet;

use grid::Grid;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (parse, cols, start) = parse(input);
    let grid = Grid::from_vec(parse, cols);

    let mut dir = Dir::Top;

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut current_pos = start;
    let mut next_pos = get_next(&dir, &current_pos);
    while let Some(next_celltype) = grid.get(next_pos.0, next_pos.1) {
        match next_celltype {
            CellType::Wall => dir = turn_90(&dir),
            _ => {
                current_pos = next_pos;
                visited.insert(current_pos);
            }
        };

        next_pos = get_next(&dir, &current_pos);
    }

    Ok(visited.len().to_string())
}

fn turn_90(curr_dir: &Dir) -> Dir {
    match curr_dir {
        Dir::Top => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Top,
    }
}

fn get_next(next_dir: &Dir, current: &(isize, isize)) -> (isize, isize) {
    match next_dir {
        Dir::Top => (current.0 - 1, current.1),
        Dir::Right => (current.0, current.1 + 1),
        Dir::Down => (current.0 + 1, current.1),
        Dir::Left => (current.0, current.1 - 1),
    }
}

enum Dir {
    Top,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum CellType {
    Clear,
    Wall,
    Start,
}

fn parse(input: &str) -> (Vec<CellType>, usize, (isize, isize)) {
    let mut start = (0, 0);
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(ri, line)| {
                line.chars()
                    .enumerate()
                    .map(|(ci, char)| match char {
                        '#' => CellType::Wall,
                        '^' => {
                            start = (ri as isize, ci as isize);
                            CellType::Start
                        }
                        _ => CellType::Clear,
                    })
                    .collect::<Vec<CellType>>()
            })
            .collect(),
        input.lines().next().unwrap().chars().count(),
        start,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
