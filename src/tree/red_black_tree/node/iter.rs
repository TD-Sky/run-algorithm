use super::Node;
use std::array;
use std::vec;

pub(super) struct ChildsIter<'a, K, V>
where
    K: Ord,
{
    inner: array::IntoIter<Option<&'a Box<Node<K, V>>>, 2>,
}

// 遍历非空子节点的方法
impl<K, V> Node<K, V>
where
    K: Ord,
{
    pub(super) fn childs(&self) -> ChildsIter<'_, K, V> {
        ChildsIter {
            inner: [self.left.as_ref(), self.right.as_ref()].into_iter(),
        }
    }
}

impl<'a, K, V> Iterator for ChildsIter<'a, K, V>
where
    K: Ord,
{
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|opt_node| match opt_node {
            None => self.next(),
            Some(node) => Some(node.as_ref()),
        })
    }
}

/* 对外的迭代器类型 */

// 前序遍历迭代器
pub struct PreorderIter<'a, K, V>
where
    K: Ord,
{
    inner: vec::IntoIter<(&'a K, &'a V)>,
}

impl<'a, K, V> PreorderIter<'a, K, V>
where
    K: Ord,
{
    pub(super) fn new(root: &'a Node<K, V>) -> Self {
        let mut nodes = Vec::new();

        Self::preorder(root, &mut nodes);

        Self {
            inner: nodes.into_iter(),
        }
    }

    fn preorder(node: &'a Node<K, V>, nodes: &mut Vec<(&'a K, &'a V)>) {
        nodes.push((&node.key, &node.value));
        for child in node.childs() {
            Self::preorder(child, nodes);
        }
    }
}

// 中序遍历迭代器
pub struct InorderIter<'a, K, V>
where
    K: Ord,
{
    inner: vec::IntoIter<(&'a K, &'a V)>,
}

impl<'a, K, V> InorderIter<'a, K, V>
where
    K: Ord,
{
    pub(super) fn new(root: &'a Node<K, V>) -> Self {
        let mut nodes = Vec::new();

        Self::inorder(root, &mut nodes);

        Self {
            inner: nodes.into_iter(),
        }
    }

    fn inorder(node: &'a Node<K, V>, nodes: &mut Vec<(&'a K, &'a V)>) {
        node.left.as_ref().map(|child| Self::inorder(child, nodes));
        nodes.push((&node.key, &node.value));
        node.right.as_ref().map(|child| Self::inorder(child, nodes));
    }
}

/* 对外迭代器方法 */

impl<'a, K, V> Iterator for PreorderIter<'a, K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> Iterator for InorderIter<'a, K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
