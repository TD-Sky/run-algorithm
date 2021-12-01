pub mod iterator;

use self::iterator::PreorderIter;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;

pub(super) enum Color {
    Red,
    Black,
}

pub(super) struct Vertex<'a, V> {
    color: Color,
    key: u32,
    value: V,
    left: Option<Box<Vertex<'a, V>>>,
    right: Option<Box<Vertex<'a, V>>>,
    marker: PhantomData<&'a V>,
}

impl Color {
    fn is_red(&self) -> bool {
        match self {
            &Color::Red => true,
            &Color::Black => false,
        }
    }
}

impl<'a, V> Vertex<'a, V> {
    fn is_red(node: &Option<Box<Vertex<'a, V>>>) -> bool {
        // 新节点必然为红
        node.as_ref().map_or(true, |node| node.color.is_red())
    }

    fn rot_left(&mut self) {
        let mut right = self.right.take().unwrap();
        self.right = right.left.take();
        mem::swap(self, right.as_mut());
        self.left = Some(right);
    }

    fn rot_right(&mut self) {
        let mut left = self.left.take().unwrap();
        self.left = left.right.take();
        mem::swap(self, left.as_mut());
        self.right = Some(left);
    }

    fn red_at_right(&self) -> bool {
        Self::is_red(&self.right) && !Self::is_red(&self.left)
    }

    fn has_double_left_red(&self) -> bool {
        Self::is_red(&self.left)
            && self
                .left
                .as_ref()
                .map_or(false, |node| Self::is_red(&node.left))
    }

    fn has_red_sides(&self) -> bool {
        Self::is_red(&self.left) && Self::is_red(&self.right)
    }

    fn flip_color(&mut self) {
        self.color = Color::Red;
        self.left.as_mut().map(|node| node.color = Color::Black);
        self.right.as_mut().map(|node| node.color = Color::Black);
    }
}

impl<'a, V> Vertex<'a, V> {
    pub(super) fn new(key: u32, value: V, color: Color) -> Self {
        Self {
            color,
            key,
            value,
            left: None,
            right: None,
            marker: PhantomData,
        }
    }

    pub(super) fn blacken(&mut self) {
        self.color = Color::Black;
    }

    pub(super) fn insert(&mut self, key: u32, value: V) {
        match self.key.cmp(&key) {
            Ordering::Equal => self.value = value,
            Ordering::Less => match &mut self.right {
                Some(vertex) => vertex.insert(key, value),
                None => self.right = Some(Box::new(Vertex::new(key, value, Color::Red))),
            },
            Ordering::Greater => match &mut self.left {
                Some(vertex) => vertex.insert(key, value),
                None => self.left = Some(Box::new(Vertex::new(key, value, Color::Red))),
            },
        }

        if self.red_at_right() {
            self.rot_left();
        }

        if self.has_double_left_red() {
            self.rot_right();
        }

        if self.has_red_sides() {
            self.flip_color();
        }
    }
}

impl<'a, V> Vertex<'a, V> {
    pub(super) fn preorder(&'a self) -> PreorderIter<'a, V> {
        PreorderIter::new(self)
    }
}
