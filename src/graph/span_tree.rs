use std::fmt::Display;
use crate::graph::{Edge, Graph};

pub fn prims_mst<'a, T>(g: &'a Graph<'a, T>) -> Vec<&'a Edge<'a, T>>
    where
        T: Eq + Display,
{
    todo!()
}

pub fn kruskals_mst<'a, T: Eq + Display >(g: &'a Graph<'a, T>) -> Vec<&'a Edge<'a, T>> {
    todo!()
}