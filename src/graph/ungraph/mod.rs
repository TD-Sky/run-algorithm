use super::VertNotInGraph;
use mst::{KruskalMST, LazyPrimMST, PrimMST};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

mod mst;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
struct Vertex<V = ()> {
    value: V,
    adj: Vec<Rc<Edge>>,
}

#[allow(dead_code)]
#[derive(Debug, Hash, PartialEq, Eq)]
struct Edge {
    start: u32,
    end: u32,
    weight: i32,
}

#[allow(dead_code)]
pub struct UnGraph<'a, V = ()> {
    adj_table: HashMap<u32, Vertex<V>>,
    marker: PhantomData<&'a V>,
}

impl<V> Vertex<V> {
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
impl<'a, V> UnGraph<'a, V> {
    fn new() -> Self {
        Self {
            adj_table: HashMap::new(),
            marker: PhantomData,
        }
    }

    fn add_vert(&mut self, vid: u32, val: V) {
        self.adj_table.entry(vid).or_insert(Vertex::new(val));
    }

    fn add_edge(&mut self, edge: (u32, u32, i32), start: V, end: V) {
        self.add_vert(edge.0, start);
        self.add_vert(edge.1, end);

        let adj_edge = Rc::new(Edge::new(edge));
        self.adj_table
            .entry(edge.0)
            .and_modify(|vert| vert.adj.push(Rc::clone(&adj_edge)));
        self.adj_table
            .entry(edge.1)
            .and_modify(|vert| vert.adj.push(Rc::clone(&adj_edge)));
    }

    fn adj_edges(&'a self, vid: u32) -> Result<&'a Vec<Rc<Edge>>, VertNotInGraph> {
        match self.adj_table.get(&vid) {
            Some(vert) => Ok(&vert.adj),
            None => Err(VertNotInGraph(vid)),
        }
    }

    fn edges(&'a self) -> HashSet<Rc<Edge>> {
        let mut edges = HashSet::new();
        for vert in self.adj_table.values() {
            for edge in &vert.adj {
                edges.insert(Rc::clone(edge));
            }
        }
        edges
    }

    fn vids(&self) -> impl Iterator<Item = u32> + '_ {
        self.adj_table.keys().map(|key| *key)
    }

    fn vs(&self) -> usize {
        self.adj_table.len()
    }

    fn lazy_prim_mst(&self, root: u32) -> Result<Vec<Weak<Edge>>, VertNotInGraph> {
        self.adj_table
            .contains_key(&root)
            .then_some(LazyPrimMST::new(self).span(root))
            .ok_or(VertNotInGraph(root))
    }

    fn prim_mst(&self, root: u32) -> Result<Vec<Weak<Edge>>, VertNotInGraph> {
        self.adj_table
            .contains_key(&root)
            .then_some(PrimMST::new(self).span(root))
            .ok_or(VertNotInGraph(root))
    }

    fn kruskal_mst(&self) -> Vec<Weak<Edge>> {
        KruskalMST::new(self).span()
    }
}
