use super::{Edge, NodeID, NodeNotInGraph};
use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(test)]
mod tests;

struct Node<V> {
    #[allow(dead_code)]
    element: V,
    neighbours: Vec<u32>,
}

impl<V> Node<V> {
    fn new(elt: V) -> Self {
        Self {
            element: elt,
            neighbours: Vec::new(),
        }
    }
}

pub struct DiGraph<V = ()> {
    adj_table: HashMap<NodeID, Node<V>>,
}

// private
impl<V> DiGraph<V> {
    fn add_node(&mut self, id: NodeID, node: V) {
        self.adj_table.entry(id).or_insert(Node::new(node));
    }

    fn neighbours(&self, id: NodeID) -> &[NodeID] {
        self.adj_table
            .get(&id)
            .map(|node| node.neighbours.as_slice())
            .unwrap()
    }

    fn bfs(&self, src: NodeID, edge_to: &mut HashMap<NodeID, NodeID>) {
        let mut marked = HashSet::with_capacity(self.node_count());
        let mut queue = VecDeque::new();

        marked.insert(src);
        queue.push_front(src);

        // 搜索会遍历所有节点
        while let Some(id) = queue.pop_back() {
            for &neighbour in self.neighbours(id) {
                // 若相邻点未标记，则压入队列
                if marked.insert(neighbour) {
                    // 核心功能：构建邻接边表
                    edge_to.insert(neighbour, id);
                    // 立即探索邻接点
                    queue.push_front(neighbour);
                }
            }
        }
    }
}

// pub
impl<V> DiGraph<V> {
    pub fn new() -> Self {
        Self {
            adj_table: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge, start: V, end: V) {
        self.add_node(edge.0, start);
        self.add_node(edge.1, end);

        self.adj_table.entry(edge.0).and_modify(|node| {
            node.neighbours.push(edge.1);
        });
    }

    pub fn contains_id(&self, id: NodeID) -> bool {
        self.adj_table.contains_key(&id)
    }

    pub fn node_count(&self) -> usize {
        self.adj_table.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adj_table
            .iter()
            .map(|(_, node)| node.neighbours.len())
            .sum()
    }

    pub fn shortest_path(
        &self,
        src: NodeID,
        dest: NodeID,
    ) -> Result<VecDeque<NodeID>, NodeNotInGraph> {
        // 确认起点和终点都存在
        if let Some(id) = [src, dest].into_iter().find(|&id| !self.contains_id(id)) {
            return Err(NodeNotInGraph(id));
        }

        let mut edge_to = HashMap::with_capacity(self.node_count());

        self.bfs(src, &mut edge_to);

        let mut path = VecDeque::new();
        let mut id = dest;

        // 在广搜得到的路径上，回溯至起点
        while id != src {
            path.push_front(id);
            id = *edge_to.get(&id).unwrap();
        }

        path.push_front(src);

        return Ok(path);
    }
}
