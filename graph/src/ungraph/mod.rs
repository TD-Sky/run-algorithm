use self::mst::{kruskal, lazy_prim, prim};
use super::{Edge, NodeID, NodeNotInGraph, WeiEdge, Weight};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

mod mst;

#[cfg(test)]
mod tests;

struct Node<V> {
    #[allow(dead_code)]
    element: V,
    adj_edges: Vec<Rc<WeiEdge>>,
}

pub struct UnGraph<V = ()> {
    adj_table: HashMap<NodeID, Node<V>>,
}

impl<V> Node<V> {
    fn new(element: V) -> Self {
        Self {
            element,
            adj_edges: Vec::new(),
        }
    }
}

impl<V> UnGraph<V> {
    fn add_node(&mut self, id: NodeID, elt: V) {
        self.adj_table.entry(id).or_insert(Node::new(elt));
    }

    fn adj_edges(&self, id: NodeID) -> &[Rc<WeiEdge>] {
        self.adj_table.get(&id).unwrap().adj_edges.as_slice()
    }
}

impl<V> UnGraph<V> {
    pub fn new() -> Self {
        Self {
            adj_table: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, weight: Weight, edge: Edge, start: V, end: V) {
        self.add_node(edge.0, start);
        self.add_node(edge.1, end);

        let wei_edge = Rc::new(WeiEdge::new(weight, edge));

        self.adj_table
            .entry(edge.0)
            .and_modify(|node| node.adj_edges.push(Rc::clone(&wei_edge)));
        self.adj_table
            .entry(edge.1)
            .and_modify(|node| node.adj_edges.push(Rc::clone(&wei_edge)));
    }

    pub fn edges(&self) -> HashSet<&WeiEdge> {
        let mut edges = HashSet::new();

        for node in self.adj_table.values() {
            for edge in &node.adj_edges {
                edges.insert(edge.as_ref());
            }
        }

        edges
    }

    pub fn ids(&self) -> impl Iterator<Item = NodeID> + '_ {
        self.adj_table.keys().map(|&id| id)
    }

    pub fn node_count(&self) -> usize {
        self.adj_table.len()
    }

    pub fn contains_id(&self, id: NodeID) -> bool {
        self.adj_table.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.adj_table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.adj_table.is_empty()
    }
}

// 生成树方法
impl<V> UnGraph<V> {
    pub fn lazy_prim_mst(&self, root: NodeID) -> Result<Vec<&'_ WeiEdge>, NodeNotInGraph> {
        self.contains_id(root)
            .then(|| lazy_prim::span(self, root))
            .ok_or(NodeNotInGraph(root))
    }

    pub fn prim_mst(&self, root: NodeID) -> Result<Vec<&'_ WeiEdge>, NodeNotInGraph> {
        self.contains_id(root)
            .then(|| prim::span(self, root))
            .ok_or(NodeNotInGraph(root))
    }

    pub fn kruskal_mst(&self) -> Option<Vec<&'_ WeiEdge>> {
        (!self.is_empty()).then(|| kruskal::span(self))
    }
}
