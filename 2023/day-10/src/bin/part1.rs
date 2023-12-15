use std::{collections::HashMap, fs};

use day10::{input_to_grid, Graph, Node};

fn main() {
    let contents = fs::read_to_string("./day-10/input-test.txt").expect("Should read file");
    println!("=========");
    println!("Result: {}", process(&contents))
}
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
//
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
fn process(content: &str) -> u64 {
    create_graph(content)
}

fn create_graph(content: &str) -> u64 {
    let grid = input_to_grid(content);
    let edges: HashMap<Node, Vec<Node>> = grid
        .iter()
        .map(|node| {
            let node_neighbors = neighboring_edges(node.value);
            let neighbors: Vec<Node> = node_neighbors
                .iter()
                .filter_map(|next_pos| {
                    let pos_next_node = Node {
                        position: (node.position.0 + next_pos.0, node.position.1 + next_pos.1),
                        value: '.',
                    };
                    grid.get(&pos_next_node).cloned()
                })
                .collect();
            (*node, neighbors)
        })
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .collect();
    dbg!(edges);
    1
}

fn neighboring_edges(me: char) -> Vec<(i32, i32)> {
    match me {
        '|' => vec![(0, 1), (0, -1)],
        '-' => vec![(1, 0), (-1, 0)],
        'L' => vec![(0, 1), (1, 0)],
        'J' => vec![(0, 1), (-1, 0)],
        '7' => vec![(0, -1), (-1, 0)],
        'F' => vec![(0, -1), (1, 0)],
        'S' => vec![(0, 1), (1, 0), (-1, 0), (-1, 0)],
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
