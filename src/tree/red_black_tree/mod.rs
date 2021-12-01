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
        match &mut self.root {
            None => self.root = Some(Box::new(Vertex::new(key, value, Color::Black))),
            Some(vertex) => {
                vertex.as_mut().insert(key, value);
                vertex.blacken();
            }
        }
    }

    pub fn preorder(&self) -> Option<iterator::PreorderIter<V>> {
        self.root.as_ref().map(|vertex| vertex.as_ref().preorder())
    }
}
