use std::collections::{HashMap, HashSet, VecDeque};

pub fn input_to_grid(content: &str) -> HashSet<Node> {
    content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| Node {
                position: (
                    x.try_into().expect("convert z to i32"),
                    y.try_into().expect("convert y to i32"),
                ),
                value: char,
            })
        })
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Node {
    pub position: (i32, i32),
    pub value: char,
}

// { "A", { "B" } },
// { "B", { "A", "C", "D" } },
// { "C", { "A" } },
// { "D", { "E", "A" } },
// { "E", { "B" } }
pub struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    pub fn bfs(&self, start: Node) -> HashMap<Node, Option<Node>> {
        let mut frontier = VecDeque::new();
        frontier.push_front(start);

        let mut came_from: HashMap<Node, Option<Node>> = HashMap::new();
        came_from.insert(start, None);
        while !frontier.is_empty() {
            if let Some(current) = frontier.pop_front() {
                println!("Visiting {:?}", &current);
                self.neighbors(current).iter().for_each(|next| {
                    if came_from.get(next).is_none() {
                        frontier.push_front(*next);
                        came_from.insert(*next, Some(current));
                    }
                })
            }
        }

        came_from
    }

    pub fn bfs_longest(&self, start: Node) -> usize {
        todo!()
    }

    fn neighbors(&self, node: Node) -> Vec<Node> {
        return self.edges.get(&node).unwrap().clone();
    }
}
