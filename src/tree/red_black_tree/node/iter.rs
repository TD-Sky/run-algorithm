use super::Node;
use std::array;
use std::vec;

pub(super) struct ChildsIter<'a, K, V>(array::IntoIter<Option<&'a Box<Node<'a, K, V>>>, 2>)
where
    K: Ord;

impl<'a, K, V> Node<'a, K, V>
where
    K: Ord,
{
    pub(super) fn childs(&'a self) -> ChildsIter<'a, K, V> {
        ChildsIter([self.left.as_ref(), self.right.as_ref()].into_iter())
    }
}

impl<'a, K, V> Iterator for ChildsIter<'a, K, V>
where
    K: Ord,
{
    type Item = &'a Node<'a, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(|opt_node| match opt_node {
            None => self.next(),
            Some(node) => Some(node.as_ref()),
        })
    }
}

/* 对外的迭代器类型 */

pub struct PreorderIter<'a, K, V>(vec::IntoIter<(&'a K, &'a V)>)
where
    K: Ord;

pub struct InorderIter<'a, K, V>(vec::IntoIter<(&'a K, &'a V)>)
where
    K: Ord;

impl<'a, K, V> PreorderIter<'a, K, V>
where
    K: Ord,
{
    pub(super) fn with_capacity(root: &'a Node<'a, K, V>, cap: usize) -> Self {
        let mut nodes = Vec::with_capacity(cap);
        Self::preorder(root, &mut nodes);
        Self(nodes.into_iter())
    }

    fn preorder(node: &'a Node<'a, K, V>, nodes: &mut Vec<(&'a K, &'a V)>) {
        nodes.push((&node.key, &node.value));
        for child in node.childs() {
            Self::preorder(child, nodes);
        }
    }
}

impl<'a, K, V> InorderIter<'a, K, V>
where
    K: Ord,
{
    pub(super) fn with_capacity(root: &'a Node<'a, K, V>, cap: usize) -> Self {
        let mut nodes = Vec::with_capacity(cap);
        Self::inorder(root, &mut nodes);
        Self(nodes.into_iter())
    }

    fn inorder(node: &'a Node<'a, K, V>, nodes: &mut Vec<(&'a K, &'a V)>) {
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
        self.0.next()
    }
}

impl<'a, K, V> Iterator for InorderIter<'a, K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
