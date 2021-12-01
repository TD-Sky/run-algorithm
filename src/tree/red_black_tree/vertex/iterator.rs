use super::Vertex;
use std::array;

struct ChildsIter<'a, V>(array::IntoIter<&'a Option<Box<Vertex<'a, V>>>, 2>);

impl<'a, V> Vertex<'a, V> {
    fn childs(&'a self) -> ChildsIter<'a, V> {
        ChildsIter([&self.left, &self.right].into_iter())
    }
}

impl<'a, V> Iterator for ChildsIter<'a, V> {
    type Item = &'a Vertex<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(|opt| match opt {
            Some(vertex) => Some(vertex.as_ref()),
            None => self.next(),
        })
    }
}

// 对外类型
pub struct PreorderIter<'a, V>(std::vec::IntoIter<(u32, &'a V)>);

impl<'a, V> PreorderIter<'a, V> {
    pub(super) fn new(root: &'a Vertex<'a, V>) -> Self {
        let mut verts = Vec::new();
        Self::preorder(root, &mut verts);
        Self(verts.into_iter())
    }

    fn preorder(vert: &'a Vertex<'a, V>, verts: &mut Vec<(u32, &'a V)>) {
        verts.push((vert.key, &vert.value));
        for child in vert.childs() {
            Self::preorder(child, verts);
        }
    }
}

// 对外迭代器方法
impl<'a, V> Iterator for PreorderIter<'a, V> {
    type Item = (u32, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
