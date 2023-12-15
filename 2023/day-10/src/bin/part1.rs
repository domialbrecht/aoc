use std::{collections::HashMap, fs};

use day10::{input_to_char_grid, Graph};
use glam::IVec2;

fn main() {
    let contents = fs::read_to_string("./day-10/input-test.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}
fn process(content: &str) -> u64 {
    let (graph, startpos) = create_graph(content);
    draw_grid_with_graph(
        content
            .lines()
            .next()
            .unwrap()
            .chars()
            .count()
            .try_into()
            .expect("Grid out of bounds"),
        &graph,
    );
    64
}

fn create_graph(content: &str) -> (Graph, IVec2) {
    let grid = input_to_char_grid(content);
    let mut startpos: Option<IVec2> = None;

    let edges: HashMap<IVec2, Vec<IVec2>> = grid
        .iter()
        .map(|(pos, value)| {
            let node_neighbors = neighboring_edges(*value);
            if *value == 'S' {
                startpos = Some(*pos);
            }
            let neighbors: Vec<IVec2> = node_neighbors
                .iter()
                .filter_map(|next_pos| {
                    let pos_next_node = IVec2::new(pos.x + next_pos.x, pos.y + next_pos.y);
                    let pos_next_node = grid.get_key_value(&pos_next_node);
                    pos_next_node.and_then(
                        |(vec, char)| {
                            if char != &'.' {
                                Some(*vec)
                            } else {
                                None
                            }
                        },
                    )
                })
                .collect();
            (*pos, neighbors)
        })
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .collect();

    (Graph::from(&edges), startpos.expect("Startpos not found"))
}

fn draw_grid_with_graph(grid_square_size: i32, graph: &Graph) {
    for y in 0..grid_square_size {
        for x in 0..grid_square_size {
            let pos = IVec2::new(x, y);
            if graph.has_edge(pos) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// .....
// .S-7.
// .|.|.
// .L-J.
// .....
fn neighboring_edges(me: char) -> Vec<IVec2> {
    match me {
        'L' => vec![IVec2::new(1, 0), IVec2::new(0, -1)],
        'J' => vec![IVec2::new(-1, 0), IVec2::new(0, -1)],
        '7' => vec![IVec2::new(-1, 0), IVec2::new(0, 1)],
        'F' => vec![IVec2::new(1, 0), IVec2::new(0, 1)],
        '|' => vec![IVec2::new(0, 1), IVec2::new(0, -1)],
        '-' => vec![IVec2::new(1, 0), IVec2::new(-1, 0)],
        'S' => vec![
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
            IVec2::new(-1, 0),
        ],
        '.' => vec![],
        x => panic!("Invalid gridType {}", x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let contents = ".....
.S-7.
.|.|.
.L-J.
.....
";
        assert_eq!(process(contents), 4);
    }

    #[test]
    fn test_two() {
        let contents = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(process(contents), 8);
    }
}
