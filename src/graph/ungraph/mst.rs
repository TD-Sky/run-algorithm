use super::{Edge, UnGraph};
use crate::union_find::UF;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

pub(super) struct LazyPrimMST<'a, V = ()> {
    graph: &'a UnGraph<'a, V>,
    marked: HashSet<u32>,
    ms_tree: Vec<Weak<Edge>>,
    edge_pq: PriorityQueue<Rc<Edge>, Reverse<i32>>,
}

pub(super) struct PrimMST<'a, V = ()> {
    graph: &'a UnGraph<'a, V>,
    tree_vids: HashSet<u32>,
    edge_to: HashMap<u32, Weak<Edge>>,
    dst_to: HashMap<u32, i32>,
    vert_pq: PriorityQueue<u32, Reverse<i32>>,
}

pub(super) struct KruskalMST<'a, V = ()> {
    graph: &'a UnGraph<'a, V>,
    ms_tree: Vec<Weak<Edge>>,
    edge_pq: PriorityQueue<Rc<Edge>, Reverse<i32>>,
    uf: UF,
}

impl<'a, V> LazyPrimMST<'a, V> {
    pub(super) fn new(graph: &'a UnGraph<'a, V>) -> Self {
        Self {
            graph,
            marked: HashSet::with_capacity(graph.vs()),
            ms_tree: Vec::with_capacity(graph.vs() - 1),
            edge_pq: PriorityQueue::new(),
        }
    }

    pub(super) fn span(mut self, src: u32) -> Vec<Weak<Edge>> {
        self.marked.insert(src);
        self.visit(src);

        while let Some((min_edge, _)) = self.edge_pq.pop() {
            let vids = min_edge.end_points();

            if self.marked.contains(&vids.0) && self.marked.contains(&vids.1) {
                continue;
            }

            self.ms_tree.push(Rc::downgrade(&min_edge));
            if self.marked.insert(vids.0) {
                self.visit(vids.0);
            }
            if self.marked.insert(vids.1) {
                self.visit(vids.1);
            }
        }

        self.ms_tree
    }

    fn visit(&mut self, vid: u32) {
        for edge in self.graph.adj_edges(vid).unwrap() {
            if !self.marked.contains(&edge.other(vid)) {
                self.edge_pq.push(Rc::clone(edge), Reverse(edge.weight()));
            }
        }
    }
}

impl<'a, V> PrimMST<'a, V> {
    pub(super) fn new(graph: &'a UnGraph<'a, V>) -> Self {
        let mut dst_to = HashMap::with_capacity(graph.vs());
        for v in graph.vids() {
            dst_to.insert(v, i32::MAX);
        }

        Self {
            graph,
            tree_vids: HashSet::with_capacity(graph.vs()),
            edge_to: HashMap::with_capacity(graph.vs() - 1),
            dst_to,
            vert_pq: PriorityQueue::with_capacity(graph.vs()),
        }
    }

    pub(super) fn span(mut self, src: u32) -> Vec<Weak<Edge>> {
        self.vert_pq.push(src, Reverse(0));
        while let Some((vid, _)) = self.vert_pq.pop() {
            self.visit(vid);
        }
        self.edge_to.values().map(|e| Weak::clone(e)).collect()
    }

    fn visit(&mut self, vid: u32) {
        self.tree_vids.insert(vid);
        for edge in self.graph.adj_edges(vid).unwrap() {
            let end = edge.other(vid);

            if self.tree_vids.contains(&end) {
                continue;
            }

            if edge.weight() < *self.dst_to.get(&end).unwrap() {
                self.edge_to.insert(end, Rc::downgrade(edge));
                self.dst_to.insert(end, edge.weight());
                self.vert_pq.push(end, Reverse(edge.weight()));
            }
        }
    }
}

impl<'a, V> KruskalMST<'a, V> {
    pub(super) fn new(graph: &'a UnGraph<'a, V>) -> Self {
        let mut edge_pq = PriorityQueue::new();
        for edge in graph.edges() {
            edge_pq.push(Rc::clone(&edge), Reverse(edge.weight()));
        }

        Self {
            graph,
            ms_tree: Vec::with_capacity(graph.vs() - 1),
            edge_pq,
            uf: UF::new(&graph.vids().collect::<Vec<u32>>()),
        }
    }

    pub(super) fn span(mut self) -> Vec<Weak<Edge>> {
        while let Some((edge, _)) = self.edge_pq.pop() {
            if self.ms_tree.len() >= self.graph.vs() {
                break;
            }
            let vids = edge.end_points();
            if !self.uf.connected(vids.0, vids.1) {
                self.uf.union(vids.0, vids.1);
                self.ms_tree.push(Rc::downgrade(&edge));
            }
        }
        self.ms_tree
    }
}
