use std::fmt::Display;
use crate::graph::{Edge, Graph, Node};

pub fn dijkstra<'a, T, W>(
    g: &'a Graph<'a, T, W>,
    from: &'a Node<'a, T>,
    to: &'a Node<'a, T>,
) -> Vec<&'a Edge<'a, T, W>>
    where
        T: Eq + Display,
{
    let mut path: Vec<&Edge<T, W>> = Vec::new();
    todo!()
}

pub fn a_star<'a, T: Eq + Display, W>(
    g: &'a Graph<'a, T, W>,
    from: &'a Node<'a, T>,
    to: &'a Node<'a, T>,
) -> Vec<&'a Edge<'a, T, W>> {


    todo!()
}