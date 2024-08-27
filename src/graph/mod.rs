use std::cmp::{Eq, PartialEq};
use std::{collections::HashSet, fmt::Display};
use std::fmt::{Debug, Formatter};

pub mod search;
pub mod path;
pub mod span_tree;

// type NodeValue = Eq + PartialEq + Display;
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

pub struct Edge<'a, T: Eq + Display, W> {
    subject: &'a Node<'a, T>,
    object: &'a Node<'a, T>,
    weight: W
}
impl<'a, T: Eq + Display, W> Edge<'a, T, W> where W: Default {

    pub fn new(from: &'a Node<'a, T>, to: &'a Node<'a, T>) -> Self {
        Edge {
            subject: from,
            object: to,
            weight: W::default()
        }
    }
    pub fn new_weighted(from: &'a Node<'a, T>, to: &'a Node<'a, T>, weight: W) -> Self {
        Edge {
            subject: from,
            object: to,
            weight: weight
        }
    }
}

pub struct Graph<'a, T: Eq + Display, W> {
    nodes: Vec<&'a Node<'a, T>>,
    edges: Vec<&'a Edge<'a, T, W>>,
}

impl<'a, T: Eq + Display, W: Debug> Display for Graph<'a, T, W>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "Node: {}", node)?;
        }
        for edge in &self.edges {
            writeln!(f, "Edge: {}-{:?}->{}", edge.subject, edge.weight, edge.object)?;
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn test_graph_init() {
        let a: Node<char> = Node::new("A", 'a');
        let b: Node<char> = Node::new("B", 'b');
        let c = Node::new("C", 'c');
        let d = Node::new("D", 'd');
        let e = Node::new("E", 'e');
        let a_to_c = Edge::new(&a, &c);
        let a_to_b = Edge::new(&a, &b);
        let b_to_d = Edge::new(&b, &d);
        let d_to_e = Edge::new(&d, &e);
        let g: Graph<char, ()> = Graph {
            nodes: vec![&a, &b, &c, &d, &e],
            edges: vec![&a_to_b, &a_to_c, &b_to_d, &d_to_e],
        };
        println!("Graph:\n{}", g);
    }
}
