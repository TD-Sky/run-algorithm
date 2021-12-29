mod node;

#[cfg(test)]
mod tests;

pub use self::node::iter;
use self::node::{Color, Node, NodePtr};

#[allow(dead_code)]
pub struct RBTreeMap<K, V>
where
    K: Ord,
{
    root: NodePtr<K, V>,
    len: usize,
}

#[allow(dead_code)]
impl<K, V> RBTreeMap<K, V>
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

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.root.is_none() {
            None
        } else {
            let opt_node: *mut NodePtr<K, V> = &mut self.root;

            // 虽然刚传入的树根节点一定是黑色的，但是旋转总会使其链接染红，
            // 代码上可以跳过
            let removal: NodePtr<K, V> = Node::remove_node(opt_node, key);

            self.root.as_mut().map(|root| root.blacken());

            removal.map(|node| {
                self.len -= 1;
                Node::into_value(node)
            })
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root
            .as_ref()
            .and_then(|root| root.get_node(key).map(|node| &node.value))
    }

    pub fn height(&self) -> Option<isize> {
        self.root.as_ref().map(|root| root.height())
    }

    pub fn preorder(&self) -> Option<iter::PreorderIter<'_, K, V>> {
        self.root.as_ref().map(|root| root.preorder())
    }

    pub fn inorder(&self) -> Option<iter::InorderIter<'_, K, V>> {
        self.root.as_ref().map(|root| root.inorder())
    }
}
