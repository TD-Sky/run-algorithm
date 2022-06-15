#![feature(let_chains, map_first_last)]

use std::cmp::Ordering;
use std::fmt;

mod digraph;
mod ungraph;

pub type NodeID = u32;
pub type Edge = (NodeID, NodeID);
pub type Weight = i32;

#[derive(Debug, Hash, Eq)]
pub struct WeiEdge {
    pub edge: Edge,
    pub weight: Weight,
}

impl PartialEq for WeiEdge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for WeiEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for WeiEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
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
