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
            Color::Red => true,
            Color::Black => false,
        }
    }

    fn rev(&mut self) {
        match self {
            Color::Red => mem::replace(self, Color::Black),
            Color::Black => mem::replace(self, Color::Red),
        };
    }

    fn replce(&mut self, src: Color) -> Self {
        mem::replace(self, src)
    }
}

impl<'a, V> Vertex<'a, V> {
    /* 摧毁叶子节点，获得其值 */
    fn into_value(self) -> V {
        self.value
    }

    /* 链接颜色判定方法 */

    fn is_red(opt: &Option<Box<Vertex<'a, V>>>) -> bool {
        // 空链接视为黑
        opt.as_ref().map_or(false, |vertex| vertex.color.is_red())
    }

    fn has_only_red_right(&self) -> bool {
        Self::is_red(&self.right) && !Self::is_red(&self.left)
    }

    fn has_red_right(&self) -> bool {
        Self::is_red(&self.right)
    }

    fn has_red_left(&self) -> bool {
        Self::is_red(&self.left)
    }

    fn has_red_double_left(&self) -> bool {
        Self::is_red(&self.left)
            && self
                .left
                .as_ref()
                .map_or(false, |left| Self::is_red(&left.left))
    }

    fn has_red_sides(&self) -> bool {
        Self::is_red(&self.left) && Self::is_red(&self.right)
    }

    fn has_red_right_left(&self) -> bool {
        self.right
            .as_ref()
            .map_or(false, |right| Self::is_red(&right.left))
    }

    /* 局部变换 */

    fn rot_left(&mut self) {
        // 拔下右节点
        let mut right = self.right.take().unwrap();

        // 中结点链接到当前节点右侧
        self.right = right.left.take();

        // 链接颜色旋转
        right.color = self.color.replce(Color::Red);

        // 交换节点指针所指堆空间
        mem::swap(self, right.as_mut());

        // 衔接节点
        self.left = Some(right);
    }

    fn rot_right(&mut self) {
        let mut left = self.left.take().unwrap();
        self.left = left.right.take();
        left.color = self.color.replce(Color::Red);
        mem::swap(self, left.as_mut());
        self.right = Some(left);
    }

    fn flip_color(&mut self) {
        self.color.rev();
        self.left.as_mut().map(|left| left.color.rev());
        self.right.as_mut().map(|right| right.color.rev());
    }

    /* fn move_red_left(&mut self) {
        self.flip_color();

        if self.has_red_right_left() {
            self.right.as_mut().unwrap().rot_right();
        }
    } */

    fn rebalance(&mut self) {
        if self.has_only_red_right() {
            self.rot_left();
        }

        if self.has_red_double_left() {
            self.rot_right();
        }

        if self.has_red_sides() {
            self.flip_color();
        }
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
            Ordering::Less => match self.right.as_mut() {
                Some(right) => right.insert(key, value),
                None => self.right = Some(Box::new(Vertex::new(key, value, Color::Red))),
            },
            Ordering::Greater => match self.left.as_mut() {
                Some(left) => left.insert(key, value),
                None => self.left = Some(Box::new(Vertex::new(key, value, Color::Red))),
            },
        }

        self.rebalance();
    }

    /* pub(super) fn pop_min(&mut self) -> Option<V> {
        self.left.as_mut().map_or(None, |left| {
            if !(self.has_red_left() || left.has_red_left()) {
                self.move_red_left();
            }
            left.pop_min();
            self.rebalance();
            unimplemented!();
        })
    } */

    /* pub(super) fn remove(&mut self, key: u32) -> Option<V> {
        let res = if self.key < key {
            if self.has_red_double_left() {
                self.move_red_left();
            }

            self.left.as_mut().unwrap().remove(key)
        } else {
            if self.has_red_left() {
                self.rot_right();
            }

            if self.key == key && self.right.is_none() {
                return None;
            }

            if !(self.has_red_right() || self.has_red_right_left()) {
                self.move_red_right();
            }

            if self.key == key {
                unimplemented!();
            } else {
                self.right.as_mut().unwrap().remove(key)
            }
        };

        self.rebalance();
        return res;
    } */
}

// 迭代器方法
impl<'a, V> Vertex<'a, V> {
    pub(super) fn preorder(&'a self) -> PreorderIter<'a, V> {
        PreorderIter::new(self)
    }
}
