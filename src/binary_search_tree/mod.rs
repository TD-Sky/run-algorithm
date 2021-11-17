use vertex::Vertex;
use vertex::{InorderIter, PreorderIter};

#[cfg(test)]
mod tests;

mod vertex;

pub struct BSTMap<'a, V> {
    root: Option<Box<Vertex<'a, V>>>,
}

#[allow(dead_code)]
impl<'a, V> BSTMap<'a, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from_preorder(verts: Vec<(u32, V)>) -> Self {
        let mut bst_map = Self { root: None };
        for (key, val) in verts {
            bst_map.insert(key, val);
        }
        bst_map
    }

    pub fn insert(&mut self, key: u32, value: V) {
        match &mut self.root {
            Some(vertex) => vertex.as_mut().insert(key, value),
            None => self.root = Some(Box::new(Vertex::new(key, value))),
        }
    }

    pub fn contains_key(&self, key: u32) -> bool {
        self.root
            .as_ref()
            .map_or(false, |vertex| vertex.as_ref().contains_key(key))
    }

    pub fn get(&self, key: u32) -> Option<&V> {
        self.root.as_ref().map(|vertex| vertex.as_ref().get(key))?
    }

    pub fn preorder(&self) -> Option<PreorderIter<V>> {
        self.root.as_ref().map(|vertex| vertex.as_ref().preorder())
    }

    pub fn inorder(&self) -> Option<InorderIter<V>> {
        self.root.as_ref().map(|vertex| vertex.as_ref().inorder())
    }
}
