mod digraph;
mod ungraph;

use std::fmt;

#[derive(Debug)]
pub struct VertNotInGraph(u32);

impl fmt::Display for VertNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertex {} is not in the graph", self.0)
    }
}

pub use digraph::DiGraph;
pub use ungraph::UnGraph;
