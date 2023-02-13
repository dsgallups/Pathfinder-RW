use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    value: i32,
}

impl Node {
    fn new(value: i32) -> Node {
        Node { value }
    }
}

struct Graph {
    nodes: HashMap<Node, (Vec<Node>, Vec<Node>)>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        self.nodes.entry(from.clone()).or_default().push(to.clone());
        self.nodes.entry(to.clone()).or_default().push(from.clone());
    }
}

fn main() {
    let mut graph = Graph::new();

    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);

    graph.add_edge(node1.clone(), node2.clone());
    graph.add_edge(node2.clone(), node3.clone());

    println!("{:?}", graph);
}
