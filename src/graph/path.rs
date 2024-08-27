use std::fmt::Display;
use crate::graph::{Edge, Graph, Node};

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

pub fn a_star<'a, T: Eq + Display>(
    g: &'a Graph<'a, T>,
    from: &'a Node<'a, T>,
    to: &'a Node<'a, T>,
) -> Vec<&'a Edge<'a, T>> {


    todo!()
}