mod vertex;

#[cfg(test)]
mod tests;

pub use self::vertex::iterator;
use self::vertex::{Color, Vertex};

#[allow(dead_code)]
pub struct RBTMap<'a, V> {
    root: Option<Box<Vertex<'a, V>>>,
}

#[allow(dead_code)]
impl<'a, V> RBTMap<'a, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: u32, value: V) {
        match self.root.as_mut() {
            None => self.root = Some(Box::new(Vertex::new(key, value, Color::Black))),
            Some(root) => {
                root.insert(key, value);
                root.blacken();
            }
        }
    }

    /* pub fn remove(&mut self, key: u32) -> Option<V> {
        self.root.as_mut().map(|vertex| vertex.remove(key))
    } */

    /* pub fn pop_min(&mut self) -> Option<V> {
        self.root.as_mut().map(|root| root.pop_min())
    } */

    pub fn preorder(&self) -> Option<iterator::PreorderIter<V>> {
        self.root.as_ref().map(|root| root.preorder())
    }
}
