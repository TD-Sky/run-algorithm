use crate::{NodeID, UnGraph, WeiEdge, Weight};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

struct PrimMST<'a, V> {
    graph: &'a UnGraph<V>,
    marked: HashSet<NodeID>,
    edge_to: HashMap<NodeID, &'a WeiEdge>,
    pq: PriorityQueue<NodeID, Reverse<Weight>>,
}

impl<'a, V> PrimMST<'a, V> {
    fn new(graph: &'a UnGraph<V>) -> Self {
        Self {
            graph,
            marked: HashSet::with_capacity(graph.node_count()),
            edge_to: HashMap::with_capacity(graph.node_count() - 1),
            pq: PriorityQueue::with_capacity(graph.node_count()),
        }
    }

    fn visit(&mut self, start: NodeID) {
        for edge in self.graph.adj_edges(start) {
            let end = edge.other(start);

            // 跳过生成树内边
            if self.marked.contains(&end) {
                continue;
            }

            // 替换成权重更小的边
            self.edge_to
                .entry(end)
                .and_modify(|this| {
                    if this.weight > edge.weight {
                        *this = edge;

                        // 记录到终点的权重
                        // 每个节点都关联唯一最小权重，失效边不会留存
                        self.pq.push(end, Reverse(edge.weight));
                    }
                })
                .or_insert_with(|| {
                    self.pq.push(end, Reverse(edge.weight));
                    edge
                });
        }
    }
}

pub(in crate::ungraph) fn span<'a, V>(graph: &'a UnGraph<V>, root: NodeID) -> Vec<&'a WeiEdge> {
    let mut mst = PrimMST::new(graph);

    mst.pq.push(root, Reverse(0)); // 使用0权重启动节点优先队列

    // 不断访问最近节点
    while let Some((closest, _)) = mst.pq.pop() {
        mst.marked.insert(closest); // 将最近节点加入生成树
        mst.visit(closest);
    }

    mst.edge_to.drain().map(|(_, edge)| edge).collect()
}
