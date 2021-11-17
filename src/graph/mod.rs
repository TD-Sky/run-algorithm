mod digraph;
mod ungraph;

use std::fmt;

#[derive(Debug, Clone)]
pub struct VertNotInGraph;

impl fmt::Display for VertNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
