use std::{
    char,
    collections::{HashMap, VecDeque},
};

use glam::IVec2;

pub fn input_to_char_grid(content: &str) -> HashMap<IVec2, char> {
    content
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| {
                (
                    IVec2::new(
                        x.try_into().expect("convert z to i32"),
                        y.try_into().expect("convert y to i32"),
                    ),
                    char,
                )
            })
        })
        .collect()
}

// { "A", { "B" } },
// { "B", { "A", "C", "D" } },
// { "C", { "A" } },
// { "D", { "E", "A" } },
// { "E", { "B" } }
#[derive(Debug)]
pub struct Graph {
    edges: HashMap<IVec2, Vec<IVec2>>,
}

impl Graph {
    pub fn from(input: &HashMap<IVec2, Vec<IVec2>>) -> Self {
        Graph {
            edges: input.clone(),
        }
    }
    pub fn bfs(&self, start: IVec2) -> HashMap<IVec2, Option<IVec2>> {
        let mut frontier = VecDeque::new();
        frontier.push_front(start);

        let mut came_from: HashMap<IVec2, Option<IVec2>> = HashMap::new();
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

    pub fn bfs_longest(&self, start: IVec2) -> usize {
        todo!()
    }

    pub fn has_edge(&self, node: IVec2) -> bool {
        self.edges.contains_key(&node)
    }

    fn neighbors(&self, node: IVec2) -> Vec<IVec2> {
        return self.edges.get(&node).unwrap().clone();
    }
}
