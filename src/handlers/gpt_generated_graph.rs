use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    id: usize,
    parents: Vec<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<usize, Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: usize) {
        self.nodes.insert(id, Node {
            id,
            parents: Vec::new(),
            children: Vec::new(),
        });
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(from_node) = self.nodes.get_mut(&from) {
            from_node.children.push(to);
        } else {
            panic!("Node {} does not exist in the graph.", from);
        }

        if let Some(to_node) = self.nodes.get_mut(&to) {
            to_node.parents.push(from);
        } else {
            panic!("Node {} does not exist in the graph.", to);
        }
    }
}