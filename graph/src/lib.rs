#![feature(bool_to_option)]

mod digraph;
mod ungraph;

use std::fmt;

#[derive(Debug)]
pub struct NodeNotInGraph(u32);

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertex {} is not in the graph", self.0)
    }
}

pub use digraph::DiGraph;
pub use ungraph::UnGraph;
