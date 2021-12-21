use super::NodeNotInGraph;
use mst::{KruskalMST, LazyPrimMST, PrimMST};
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

mod mst;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
struct Node<V = ()> {
    value: V,
    adj: Vec<Rc<Edge>>,
}

#[allow(dead_code)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Edge {
    start: u32,
    end: u32,
    weight: i32,
}

#[allow(dead_code)]
pub struct UnGraph<V = ()> {
    adj_table: HashMap<u32, Node<V>>,
}

impl<V> Node<V> {
    fn new(value: V) -> Self {
        Self {
            value,
            adj: Vec::new(),
        }
    }
}

impl Edge {
    fn new(edge: (u32, u32, i32)) -> Self {
        Self {
            start: edge.0,
            end: edge.1,
            weight: edge.2,
        }
    }

    fn other(&self, vid: u32) -> u32 {
        self.start + self.end - vid
    }

    fn end_points(&self) -> (u32, u32) {
        (self.start, self.end)
    }

    fn weight(&self) -> i32 {
        self.weight
    }
}

#[allow(dead_code)]
impl<V> UnGraph<V> {
    fn add_node(&mut self, vid: u32, val: V) {
        self.adj_table.entry(vid).or_insert(Node::new(val));
    }

    fn adj_edges(&self, vid: u32) -> Result<&Vec<Rc<Edge>>, NodeNotInGraph> {
        match self.adj_table.get(&vid) {
            Some(node) => Ok(&node.adj),
            None => Err(NodeNotInGraph(vid)),
        }
    }
}

#[allow(dead_code)]
impl<V> UnGraph<V> {
    pub fn new() -> Self {
        Self {
            adj_table: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, edge: (u32, u32, i32), start: V, end: V) {
        self.add_node(edge.0, start);
        self.add_node(edge.1, end);

        let adj_edge = Rc::new(Edge::new(edge));
        self.adj_table
            .entry(edge.0)
            .and_modify(|node| node.adj.push(Rc::clone(&adj_edge)));
        self.adj_table
            .entry(edge.1)
            .and_modify(|node| node.adj.push(Rc::clone(&adj_edge)));
    }

    pub fn edges(&self) -> HashSet<Rc<Edge>> {
        let mut edges = HashSet::new();
        for node in self.adj_table.values() {
            for edge in &node.adj {
                edges.insert(Rc::clone(edge));
            }
        }
        edges
    }

    pub fn vids(&self) -> impl Iterator<Item = u32> + '_ {
        self.adj_table.keys().map(|key| *key)
    }

    pub fn vs(&self) -> usize {
        self.adj_table.len()
    }
}

// 生成树方法
#[allow(dead_code)]
impl<V> UnGraph<V> {
    pub fn lazy_prim_mst(&self, root: u32) -> Result<Vec<Weak<Edge>>, NodeNotInGraph> {
        self.adj_table
            .contains_key(&root)
            .then_some(LazyPrimMST::new(self).span(root))
            .ok_or(NodeNotInGraph(root))
    }

    pub fn prim_mst(&self, root: u32) -> Result<Vec<Weak<Edge>>, NodeNotInGraph> {
        self.adj_table
            .contains_key(&root)
            .then_some(PrimMST::new(self).span(root))
            .ok_or(NodeNotInGraph(root))
    }

    pub fn kruskal_mst(&self) -> Vec<Weak<Edge>> {
        KruskalMST::new(self).span()
    }
}
