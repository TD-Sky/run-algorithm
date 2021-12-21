use super::{Edge, UnGraph};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use union_find::UF;

type GhostEdge = Option<Weak<Edge>>;

pub(super) struct LazyPrimMST<'a, V = ()> {
    graph: &'a UnGraph<V>,
    marked: HashSet<u32>,
    ms_tree: Vec<Weak<Edge>>,
    edge_pq: PriorityQueue<Rc<Edge>, Reverse<i32>>,
}

pub(super) struct PrimMST<'a, V = ()> {
    graph: &'a UnGraph<V>,
    marked: HashSet<u32>,
    edge_to: HashMap<u32, GhostEdge>,
    vert_pq: PriorityQueue<u32, Reverse<i32>>,
}

pub(super) struct KruskalMST<V = ()> {
    ms_tree: Vec<Weak<Edge>>,
    edge_pq: PriorityQueue<Rc<Edge>, Reverse<i32>>,
    uf: UF,
    marker: PhantomData<V>,
}

impl<'a, V> LazyPrimMST<'a, V> {
    pub(super) fn new(graph: &'a UnGraph<V>) -> Self {
        Self {
            graph,
            marked: HashSet::with_capacity(graph.vs()),
            ms_tree: Vec::with_capacity(graph.vs() - 1),
            edge_pq: PriorityQueue::new(),
        }
    }

    pub(super) fn span(mut self, root: u32) -> Vec<Weak<Edge>> {
        self.marked.insert(root);
        self.visit(root); // 将根节点的所有邻接边加入优先队列

        while let Some((min_edge, _)) = self.edge_pq.pop() {
            let vids: (u32, u32) = min_edge.end_points();

            // 边优先队列的入队操作由标记点引发，
            // 故每次循环最多只有一个未标记点；
            // 标记最小边新端点，记录它们的邻接边；
            if self.marked.insert(vids.0) {
                self.visit(vids.0);
            } else if self.marked.insert(vids.1) {
                self.visit(vids.1);
            } else {
                // 没有新端点，说明是生成树内边，跳过
                continue;
            }

            // 将这条最小边加入生成树
            self.ms_tree.push(Rc::downgrade(&min_edge));
        }

        self.ms_tree
    }

    fn visit(&mut self, start: u32) {
        // 记录不属于生成树的边
        for edge in self.graph.adj_edges(start).unwrap() {
            if !self.marked.contains(&edge.other(start)) {
                self.edge_pq.push(Rc::clone(edge), Reverse(edge.weight()));
            }
        }
    }
}

impl<'a, V> PrimMST<'a, V> {
    pub(super) fn new(graph: &'a UnGraph<V>) -> Self {
        let mut edge_to = HashMap::with_capacity(graph.vs() - 1);
        // 标记所有边的权重为无穷大
        for vid in graph.vids() {
            edge_to.insert(vid, None);
        }

        Self {
            graph,
            marked: HashSet::with_capacity(graph.vs()),
            edge_to,
            vert_pq: PriorityQueue::with_capacity(graph.vs()),
        }
    }

    pub(super) fn span(mut self, root: u32) -> Vec<Weak<Edge>> {
        self.edge_to.remove(&root); // 不需要有边指向根节点
        self.vert_pq.push(root, Reverse(0)); // 使用0权重启动节点优先队列

        // 不断访问最近节点
        while let Some((start, _)) = self.vert_pq.pop() {
            self.marked.insert(start); // 将最近节点加入生成树
            self.visit(start);
        }

        self.edge_to.drain().map(|(_, e)| e.unwrap()).collect()
    }

    fn visit(&mut self, start: u32) {
        for edge in self.graph.adj_edges(start).unwrap() {
            let end: u32 = edge.other(start);

            // 跳过生成树内边
            if self.marked.contains(&end) {
                continue;
            }

            let ghost_edge: &mut GhostEdge = self.edge_to.get_mut(&end).unwrap();
            if Self::less_weight(ghost_edge, edge.weight()) {
                ghost_edge.replace(Rc::downgrade(edge)); // 替换成权重更小的边

                // 记录到终点的权重
                // 每个节点都关联唯一最小权重，失效边不会留存
                self.vert_pq.push(end, Reverse(edge.weight()));
            }
        }
    }

    fn less_weight(ghost_edge: &GhostEdge, weight: i32) -> bool {
        match ghost_edge {
            None => true, //None表示权重无穷大
            Some(edge) => edge.upgrade().unwrap().weight() > weight,
        }
    }
}

impl<V> KruskalMST<V> {
    pub(super) fn new(graph: &UnGraph<V>) -> Self {
        let mut edge_pq = PriorityQueue::new();
        // 准备逐条取出权重最小边
        for edge in graph.edges() {
            edge_pq.push(Rc::clone(&edge), Reverse(edge.weight()));
        }

        Self {
            // 生成森林并查集，准备逐步聚合这些连通分量
            uf: UF::new(&graph.vids().collect::<Vec<u32>>()),
            ms_tree: Vec::with_capacity(graph.vs() - 1),
            edge_pq,
            marker: PhantomData,
        }
    }

    pub(super) fn span(mut self) -> Vec<Weak<Edge>> {
        // 不断抽取图的边，直到树完整为止
        loop {
            match self.edge_pq.pop() {
                Some((edge, _)) if self.incomplete() => {
                    let vids = edge.end_points();

                    // 并查集就是在建树
                    // 点不在树中，就是说点与树中的点不连通，
                    // 收入此点也即收入这次循环的边
                    if !self.uf.connected(vids.0, vids.1) {
                        self.uf.union(vids.0, vids.1);
                        self.ms_tree.push(Rc::downgrade(&edge));
                    }
                }
                _ => return self.ms_tree,
            }
        }
    }

    // 检查树是否未生成完毕
    fn incomplete(&self) -> bool {
        self.ms_tree.len() < self.ms_tree.capacity()
    }
}
