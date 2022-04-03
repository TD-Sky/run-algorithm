#![feature(bool_to_option, let_chains)]

use std::fmt;

mod digraph;
mod ungraph;

pub type NodeID = u32;
pub type Edge = (NodeID, NodeID);
pub type Weight = i32;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WeiEdge {
    pub edge: Edge,
    pub weight: Weight,
}

impl WeiEdge {
    pub fn new(weight: Weight, edge: Edge) -> Self {
        Self { weight, edge }
    }

    pub fn other(&self, id: NodeID) -> NodeID {
        self.edge.0 + self.edge.1 - id
    }
}

#[derive(PartialEq, Debug)]
pub struct NodeNotInGraph(u32);

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "node {} is not in the graph", self.0)
    }
}

pub use self::digraph::DiGraph;
pub use self::ungraph::UnGraph;
