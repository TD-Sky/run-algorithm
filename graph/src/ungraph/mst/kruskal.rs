use crate::{UnGraph, WeiEdge};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use union_find::UF;

pub(in crate::ungraph) fn span<'a, V>(graph: &'a UnGraph<V>) -> Vec<&'a WeiEdge> {
    let mut uf = UF::from_iter(graph.ids());
    let mut pq = PriorityQueue::new();
    let branches = graph.node_count() - 1;
    let mut mst = Vec::with_capacity(branches);

    // 准备逐条取出权重最小边
    for edge in graph.edges() {
        pq.push(edge, Reverse(edge.weight));
    }

    while let Some((wei_edge, _)) = pq.pop() && mst.len() < branches {
                // 并查集就是在建树
                // 点不在树中，就是说点与树中的点不连通，
                // 收入此点也即收入这次循环的边
                if !uf.connected(wei_edge.edge.0, wei_edge.edge.1) {
                    uf.union(wei_edge.edge.0, wei_edge.edge.1);
                    mst.push(wei_edge);
                }
        }

    mst
}
