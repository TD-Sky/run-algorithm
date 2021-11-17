use std::cmp::Ordering;
use std::marker::PhantomData;

pub(super) struct Vertex<'a, V> {
    childs: [Option<Box<Vertex<'a, V>>>; 2],
    key: u32,
    value: V,
    marker: PhantomData<&'a V>,
}

impl<'a, V> Drop for Vertex<'a, V> {
    // 后序遍历摧毁树
    fn drop(&mut self) {
        for child_opt in &mut self.childs {
            if let Some(child) = child_opt {
                drop(child);
            }
        }
        drop(&mut self.value);
    }
}

struct ChildsIter<'a, V>(std::slice::Iter<'a, Option<Box<Vertex<'a, V>>>>);

impl<'a, V> Iterator for ChildsIter<'a, V> {
    type Item = &'a Vertex<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(|opt| match opt {
            Some(vertex) => Some(vertex.as_ref()),
            None => self.next(),
        })
    }
}

struct ChildsIterMut<'a, V>(std::slice::IterMut<'a, Option<Box<Vertex<'a, V>>>>);

impl<'a, V> Iterator for ChildsIterMut<'a, V> {
    type Item = &'a mut Vertex<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(|opt| match opt {
            Some(vertex) => Some(vertex.as_mut()),
            None => self.next(),
        })
    }
}

pub struct PreorderIter<'a, V>(std::vec::IntoIter<(u32, &'a V)>);

impl<'a, V> Iterator for PreorderIter<'a, V> {
    type Item = (u32, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct InorderIter<'a, V>(std::vec::IntoIter<&'a V>);

impl<'a, V> Iterator for InorderIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[allow(dead_code)]
impl<'a, V> Vertex<'a, V> {
    fn childs(&'a self) -> ChildsIter<'a, V> {
        ChildsIter(self.childs.iter())
    }

    fn childs_mut(&'a mut self) -> ChildsIterMut<'a, V> {
        ChildsIterMut(self.childs.iter_mut())
    }
}

#[allow(dead_code)]
impl<'a, V> Vertex<'a, V> {
    pub(super) fn new(key: u32, value: V) -> Self {
        Vertex {
            childs: [None, None],
            key,
            value,
            marker: PhantomData,
        }
    }

    pub(super) fn insert(&mut self, key: u32, value: V) {
        match self.key.cmp(&key) {
            Ordering::Equal => self.value = value,
            // 新节点大于当前节点，插入右边
            Ordering::Less => match &mut self.childs[1] {
                Some(vertex) => vertex.insert(key, value),
                None => self.childs[1] = Some(Box::new(Vertex::new(key, value))),
            },
            // 新节点小于当前节点，插入左边
            Ordering::Greater => match &mut self.childs[0] {
                Some(vertex) => vertex.insert(key, value),
                None => self.childs[0] = Some(Box::new(Vertex::new(key, value))),
            },
        }
    }

    pub(super) fn contains_key(&self, key: u32) -> bool {
        match self.key.cmp(&key) {
            Ordering::Equal => true,
            // 查询节点大于当前节点，向右
            Ordering::Less => self.childs[1]
                .as_ref()
                .map_or(false, |vertex| vertex.as_ref().contains_key(key)),
            // 查询节点小于当前节点，向左
            Ordering::Greater => self.childs[0]
                .as_ref()
                .map_or(false, |vertex| vertex.as_ref().contains_key(key)),
        }
    }

    pub(super) fn get(&self, key: u32) -> Option<&V> {
        match self.key.cmp(&key) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.childs[1].as_ref().map(|vertex| vertex.get(key))?,
            Ordering::Greater => self.childs[0].as_ref().map(|vertex| vertex.get(key))?,
        }
    }

    pub(super) fn preorder(&'a self) -> PreorderIter<'a, V> {
        PreorderIter::new(self)
    }

    pub(super) fn inorder(&'a self) -> InorderIter<'a, V> {
        InorderIter::new(self)
    }
}

impl<'a, V> PreorderIter<'a, V> {
    fn new(root: &'a Vertex<'a, V>) -> Self {
        let mut verts = Vec::new();
        PreorderIter::preorder(root, &mut verts);
        Self(verts.into_iter())
    }

    fn preorder(vert: &'a Vertex<'a, V>, verts: &mut Vec<(u32, &'a V)>) {
        verts.push((vert.key, &vert.value));
        for child in vert.childs() {
            PreorderIter::preorder(child, verts);
        }
    }
}

impl<'a, V> InorderIter<'a, V> {
    fn new(root: &'a Vertex<'a, V>) -> Self {
        let mut verts = Vec::new();
        InorderIter::inorder(root, &mut verts);
        Self(verts.into_iter())
    }

    fn inorder(vert: &'a Vertex<'a, V>, verts: &mut Vec<&'a V>) {
        match &vert.childs {
            [None, None] => verts.push(&vert.value),
            [Some(child), None] | [None, Some(child)] => {
                InorderIter::inorder(child.as_ref(), verts);
                verts.push(&vert.value);
            }
            [Some(left), Some(right)] => {
                InorderIter::inorder(left.as_ref(), verts);
                verts.push(&vert.value);
                InorderIter::inorder(right.as_ref(), verts);
            }
        }
    }
}
