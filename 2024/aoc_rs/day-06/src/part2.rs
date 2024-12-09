use std::collections::HashMap;

use grid::Grid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Dir {
    Top,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
enum CellType {
    Clear,
    Wall,
    Obstacle,
    Start,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (parse, cols, start) = parse(input);
    let grid = Grid::from_vec(parse, cols);

    let possible_grids = grid
        .indexed_iter()
        .fold(Vec::new(), |mut acc, ((row, col), cell)| {
            if let CellType::Clear = cell {
                let mut newgrid = grid.clone();
                *newgrid.get_mut(row, col).unwrap() = CellType::Obstacle;
                acc.push(newgrid);
            }
            acc
        });

    let stucks = possible_grids
        .iter()
        .filter(|new_grid| is_stuck(new_grid, &start));

    Ok(stucks.count().to_string())
}

fn is_stuck(grid: &Grid<CellType>, start: &Position) -> bool {
    let mut dir = Dir::Top;
    let mut hits = HashMap::new();

    let mut current_pos = start.clone();
    let mut next_pos = get_next(&dir, current_pos.clone());
    while let Some(next_celltype) = grid.get(next_pos.y, next_pos.x) {
        if hits.iter().any(|(_hit, count)| count > &1) {
            return true;
        }

        match next_celltype {
            CellType::Obstacle | CellType::Wall => {
                *hits.entry((current_pos.clone(), dir)).or_insert(0) += 1;
                dir = turn_90(&dir);
            }
            _ => {
                current_pos = next_pos;
            }
        };

        next_pos = get_next(&dir, current_pos.clone());
    }
    false
}

fn turn_90(curr_dir: &Dir) -> Dir {
    match curr_dir {
        Dir::Top => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Top,
    }
}

fn get_next(next_dir: &Dir, mut current: Position) -> Position {
    match next_dir {
        Dir::Top => current.y -= 1,
        Dir::Right => current.x += 1,
        Dir::Down => current.y += 1,
        Dir::Left => current.x -= 1,
    }

    current.clone()
}

fn parse(input: &str) -> (Vec<CellType>, usize, Position) {
    let mut start = Position { x: 0, y: 0 };
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
                            start = Position {
                                y: ri as isize,
                                x: ci as isize,
                            };
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
