use std::fmt::Display;
use crate::graph::{Edge, Graph};

pub fn prims_mst<'a, T, W>(g: &'a Graph<'a, T, W>) -> Vec<&'a Edge<'a, T, W>>
    where
        T: Eq + Display,
{
    todo!()
}

pub fn kruskals_mst<'a, T: Eq + Display, W>(g: &'a Graph<'a, T, W>) -> Vec<&'a Edge<'a, T, W>> {
    todo!()
}