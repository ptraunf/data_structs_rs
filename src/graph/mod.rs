use std::cmp::{Eq, PartialEq};
use std::{collections::HashSet, fmt::Display};

pub struct Node<'a, T: Eq + PartialEq + Display> {
    id: &'a str,
    value: T,
}

impl<'a, T: Eq + Display> Node<'a, T> {
    pub fn new(id: &'a str, value: T) -> Self {
        Self { id, value }
    }
}
impl<'a, T: Eq + Display> Display for Node<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>[{}]", self.id, self.value)
    }
}
impl<'a, T: PartialEq + Eq + Display> PartialEq for Node<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}
impl<'a, T: Eq + Display> Eq for Node<'a, T> {}

pub struct Edge<'a, T: Eq + Display> {
    subject: &'a Node<'a, T>,
    object: &'a Node<'a, T>,
}
impl<'a, T: Eq + Display> Edge<'a, T> {
    pub fn new(from: &'a Node<'a, T>, to: &'a Node<'a, T>) -> Self {
        Edge {
            subject: from,
            object: to,
        }
    }
}
pub struct Graph<'a, T: Eq + Display> {
    nodes: Vec<&'a Node<'a, T>>,
    edges: Vec<&'a Edge<'a, T>>,
}

impl<'a, T: Eq + Display> Display for Graph<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "Node: {}", node)?;
        }
        for edge in &self.edges {
            writeln!(f, "Edge: {}->{}", edge.subject, edge.object)?;
        }

        Ok(())
    }
}

pub fn depth_first_search<'a, T: Eq + Display>(
    g: &'a Graph<'a, T>,
    search: T,
) -> Option<&'a Node<'a, T>> {
    let start_node = g.nodes.first().unwrap();
    if start_node.value == search {
        return Some(start_node);
    }
    let mut nodes_visited: HashSet<&str> = HashSet::new();
    let mut neighbor_stack: Vec<&Node<T>> = Vec::new();
    neighbor_stack.push(start_node);
    while let Some(current) = neighbor_stack.pop() {
        if nodes_visited.contains(current.id) {
            println!("Back to node {}", current);
            return None;
        } else {
            println!("Visted node {}", current);
            nodes_visited.insert(current.id);
        }
        for neighbor in g
            .edges
            .iter()
            .filter(|e| e.subject == current)
            .map(|e| e.object)
        {
            if neighbor.value == search {
                return Some(neighbor);
            } else {
                neighbor_stack.push(neighbor);
            }
        }
    }
    None
}

// "Patient"
pub fn breadth_first_search<'a, T: Eq + Display>(
    g: &'a Graph<'a, T>,
    search: T,
) -> Option<&'a Node<'a, T>> {
    todo!()
}
pub fn dijkstra<'a, T>(
    g: &'a Graph<'a, T>,
    from: &'a Node<'a, T>,
    to: &'a Node<'a, T>,
) -> Vec<&'a Edge<'a, T>>
    where
        T: Eq + Display,
{
    let mut path: Vec<&Edge<T>> = Vec::new();
    todo!()
}
pub fn prims_mst<'a, T>(g: &'a Graph<'a, T>) -> Vec<&'a Edge<'a, T>>
    where
        T: Eq + Display,
{
    todo!()
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn test_depth_first_search() {
        let a: Node<char> = Node::new("A", 'a');
        let b: Node<char> = Node::new("B", 'b');
        let c = Node::new("C", 'c');
        let d = Node::new("D", 'd');
        let e = Node::new("E", 'e');
        let a_to_c = Edge::new(&a, &c);
        let a_to_b = Edge::new(&a, &b);
        let b_to_d = Edge::new(&b, &d);
        let d_to_e = Edge::new(&d, &e);
        let g: Graph<char> = Graph {
            nodes: vec![&a, &b, &c, &d, &e],
            edges: vec![&a_to_b, &a_to_c, &b_to_d, &d_to_e],
        };
        let search = 'e';
        if let Some(result) = depth_first_search(&g, search) {
            assert!(*result == e);
            println!("Found {}: Node {}", search, result);
        }
        println!("Graph:\n{}", g);
    }
}
