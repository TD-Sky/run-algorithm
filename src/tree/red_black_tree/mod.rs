mod node;

#[cfg(test)]
mod tests;

pub use self::node::iterator;
use self::node::{Color, Node};

#[allow(dead_code)]
pub struct RBTMap<'a, K, V>
where
    K: Ord,
{
    root: Option<Box<Node<'a, K, V>>>,
    len: usize,
}

#[allow(dead_code)]
impl<'a, K, V> RBTMap<'a, K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.len += 1;

        match self.root.as_mut() {
            None => {
                self.root = Some(Box::new(Node::new(key, value, Color::Black)));
                None
            }

            Some(root) => {
                let res = root.insert(key, value);

                root.blacken();

                res.map(|val| {
                    self.len -= 1;
                    val
                })
            }
        }
    }

    pub fn pop_min(&mut self) -> Option<V> {
        if self.root.is_none() {
            None
        } else {
            self.len -= 1;

            let opt_node: *mut Option<Box<Node<'a, K, V>>> = &mut self.root;

            // 虽然刚传入的树根节点一定是黑色的，但是旋转总会使其链接染红，
            // 代码上可以跳过
            let min_node: Box<Node<'a, K, V>> = Node::pop_min_node(opt_node);

            self.root.as_mut().map(|root| root.blacken());

            Some(Node::into_value(min_node))
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.root.is_none() {
            None
        } else {
            let opt_node: *mut Option<Box<Node<'a, K, V>>> = &mut self.root;

            let removal: Option<Box<Node<'a, K, V>>> = Node::remove_node(opt_node, key);

            self.root.as_mut().map(|root| root.blacken());

            removal.map(|node| {
                self.len -= 1;
                Node::into_value(node)
            })
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|root| root.get(key))
    }

    pub fn preorder(&'a self) -> Option<iterator::PreorderIter<'a, K, V>> {
        self.root.as_ref().map(|root| root.preorder(self.len))
    }

    pub fn inorder(&'a self) -> Option<iterator::InorderIter<'a, K, V>> {
        self.root.as_ref().map(|root| root.inorder(self.len))
    }
}
