use std::collections::HashSet;
use std::fmt::Display;
use crate::graph::{Edge, Graph, Node};

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
