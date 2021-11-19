use super::VertNotInGraph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;

#[allow(dead_code)]
struct Vertex<V = ()> {
    element: V,
    neighbours: Vec<u32>,
}

impl<V> Vertex<V> {
    fn new(elt: V) -> Self {
        Self {
            element: elt,
            neighbours: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub struct DiGraph<'a, V = ()> {
    adj_table: HashMap<u32, Vertex<V>>,
    marker: PhantomData<&'a V>,
}

#[allow(dead_code)]
impl<'a, V> DiGraph<'a, V> {
    fn add_vert(&mut self, vid: u32, vert: V) {
        self.adj_table.entry(vid).or_insert(Vertex::new(vert));
    }

    fn neighbours(&'a self, vid: u32) -> Result<&'a Vec<u32>, VertNotInGraph> {
        match self.adj_table.get(&vid) {
            Some(vert) => Ok(&vert.neighbours),
            None => Err(VertNotInGraph(vid)),
        }
    }

    fn bfs(&self, src: u32, marked: &mut HashSet<u32>, edge_to: &mut HashMap<u32, u32>) {
        marked.insert(src);
        let mut vid_queue: VecDeque<u32> = VecDeque::new();
        vid_queue.push_front(src);

        // 搜索会遍历所有节点
        while !vid_queue.is_empty() {
            let vid = vid_queue.pop_back().unwrap();
            for neighbour in self.neighbours(vid).unwrap() {
                if marked.insert(*neighbour) {
                    // 核心功能：构建邻接边表
                    edge_to.insert(*neighbour, vid);

                    vid_queue.push_front(*neighbour);
                }
            }
        }
    }
}

#[allow(dead_code)]
impl<'a, V> DiGraph<'a, V> {
    pub fn new() -> Self {
        Self {
            adj_table: HashMap::new(),
            marker: PhantomData,
        }
    }

    pub fn add_edge(&mut self, edge: (u32, u32), start: V, end: V) {
        self.add_vert(edge.0, start);
        self.add_vert(edge.1, end);

        self.adj_table.entry(edge.0).and_modify(|vert| {
            vert.neighbours.push(edge.1);
        });
    }

    pub fn contains_vert(&self, vid: u32) -> bool {
        self.adj_table.contains_key(&vid)
    }

    pub fn edges(&self) -> Vec<(u32, u32)> {
        let mut edges = Vec::new();
        for (vid, start) in &self.adj_table {
            for end in &start.neighbours {
                edges.push((*vid, *end));
            }
        }
        edges
    }

    pub fn vs(&self) -> usize {
        self.adj_table.len()
    }

    pub fn shortest_path(&'a self, src: u32, dest: u32) -> Option<VecDeque<u32>>
    where
        Self: Sized,
    {
        let mut marked: HashSet<u32> = HashSet::with_capacity(self.vs());
        let mut edge_to: HashMap<u32, u32> = HashMap::with_capacity(self.vs());

        self.bfs(src, &mut marked, &mut edge_to);

        marked.contains(&dest).then(|| {
            let mut path: VecDeque<u32> = VecDeque::new();
            let mut vid = dest;
            while vid != src {
                path.push_front(vid);
                vid = *edge_to.get(&vid).unwrap();
            }
            path.push_front(src);
            path
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DiGraph;
    #[test]
    fn dfs() {
        let mut digraph: DiGraph<()> = DiGraph::new();
        digraph.add_edge((0, 2), (), ());
        digraph.add_edge((0, 1), (), ());
        digraph.add_edge((0, 5), (), ());
        digraph.add_edge((3, 5), (), ());
        digraph.add_edge((3, 4), (), ());
        digraph.add_edge((2, 4), (), ());
        digraph.add_edge((2, 1), (), ());
        digraph.add_edge((2, 3), (), ());
        digraph.shortest_path(0, 5).unwrap(); // [0, 5]
    }
}
