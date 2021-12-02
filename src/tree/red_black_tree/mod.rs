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

    pub fn pop_min(&mut self) -> Option<V> {
        if self.root.is_none() {
            None
        } else {
            let opt_vert: *mut Option<Box<Vertex<'a, V>>> = &mut self.root;

            // 虽然刚传入的树根节点一定是黑色的，但是旋转总会使其链接染红，
            // 代码上可以跳过
            let min_vert: Box<Vertex<'a, V>> = Vertex::pop_min_vertex(opt_vert);

            self.root.as_mut().map(|root| root.blacken());

            Some(Vertex::into_value(min_vert))
        }
    }

    pub fn remove(&mut self, key: u32) -> Option<V> {
        if self.root.is_none() {
            None
        } else {
            let opt_vert: *mut Option<Box<Vertex<'a, V>>> = &mut self.root;

            let removal: Option<Box<Vertex<'a, V>>> = Vertex::remove_vertex(opt_vert, key);

            self.root.as_mut().map(|root| root.blacken());

            removal.map_or(None, |vert| Some(Vertex::into_value(vert)))
        }
    }

    pub fn preorder(&self) -> Option<iterator::PreorderIter<V>> {
        self.root.as_ref().map(|root| root.preorder())
    }
}
