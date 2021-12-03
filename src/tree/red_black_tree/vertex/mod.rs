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
    /* 链接颜色判定方法 */

    fn is_red(opt: &Option<Box<Vertex<'a, V>>>) -> bool {
        // 空链接视为黑
        opt.as_ref().map_or(false, |vertex| vertex.color.is_red())
    }

    fn has_red_right(&self) -> bool {
        Self::is_red(&self.right)
    }

    fn has_red_left(&self) -> bool {
        Self::is_red(&self.left)
    }

    fn has_red_double_left(&self) -> bool {
        self.left
            .as_ref()
            .map_or(false, |left| left.color.is_red() && left.has_red_left())
    }

    fn has_red_right_left(&self) -> bool {
        self.right
            .as_ref()
            .map_or(false, |right| right.has_red_left())
    }

    fn has_red_left_left(&self) -> bool {
        self.left.as_ref().map_or(false, |left| left.has_red_left())
    }

    /* 局部变换 */

    fn rot_left(&mut self) {
        // 拔下右节点
        let mut right = self.right.take().unwrap();

        // 中结点链接到当前节点右侧
        self.right = right.left.take();

        // 链接颜色旋转
        right.color = self.color.replce(Color::Red);

        // 交换节点指针所指堆空间的内容
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

    fn restruct_left(&mut self) {
        // 局部重构，使左子节点粘连
        self.flip_color();

        // 向下的树是完好的，故只可能存在红色左链接，
        // 若是如此，重构后节点会发生粘连，
        // 在2-3-4树中表现为上溢。
        if self.has_red_right_left() {
            self.right.as_mut().unwrap().rot_right();
            self.rot_left();
            // 消除粘连
            self.flip_color();
        }
    }

    fn restruct_right(&mut self) {
        self.flip_color();

        if self.has_red_left_left() {
            self.rot_right();
            self.flip_color();
        }
    }

    fn rebalance(&mut self) {
        // 其实直接判定右红也可以
        // 但这会转换成双左情况
        if self.has_red_right() && !self.has_red_left() {
            self.rot_left();
        }

        if self.has_red_double_left() {
            self.rot_right();
        }

        if self.has_red_left() && self.has_red_right() {
            self.flip_color();
        }
    }

    /* 交换键、值 */
    fn swap_successor(&mut self) {
        // 函数入口上下文：
        // - 删除键已匹配
        // - 传入节点必有右节点
        // - 右节点与传入节点交换键值后，后继结点会立刻删除

        // 寻找传入节点右子树的最小节点
        let mut successor: &mut Box<Self> = self.right.as_mut().unwrap();
        while let Some(left) = &mut successor.left {
            successor = left;
        }

        mem::swap(&mut self.key, &mut successor.key);
        mem::swap(&mut self.value, &mut successor.value);
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

    // 摧毁叶子节点，获得其值
    pub(super) fn into_value(vertex: Box<Self>) -> V {
        vertex.value
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

    pub(super) fn pop_min_vertex(opt_vert: *mut Option<Box<Self>>) -> Box<Self> {
        // 删除节点不能破坏树的平衡，因此只要删除红节点即可。
        // 为了保证总是删除红节点，我们先进行局部重构，令
        // 当前节点 或 当前节点之左 为红。
        unsafe {
            (&mut *opt_vert)
                .as_mut()
                .map(|vert| {
                    let opt_left: *mut Option<Box<Self>> = &mut vert.left;

                    match &mut *opt_left {
                        None => (&mut *opt_vert).take().unwrap(),
                        Some(left) => {
                            // 捏红节点
                            // 若节点的左和左之左都为黑，
                            // 则借取节点以拼接，使子节点形成3/4-节点。
                            // 若左之左为红，说明仍有更小的节点存在，可以直接往下；
                            // 同时，这也说明左键在一个3-节点内，无需再借用拼接。
                            if !(vert.has_red_left() || left.has_red_left()) {
                                // 2-3-4树左旋式局部重整
                                vert.restruct_left();
                            }

                            let min_vert = Self::pop_min_vertex(opt_left);

                            vert.rebalance();

                            min_vert
                        }
                    }
                })
                .unwrap()
        }
    }

    pub(super) fn remove_vertex(opt_vert: *mut Option<Box<Self>>, key: u32) -> Option<Box<Self>> {
        unsafe {
            (&mut *opt_vert).as_mut().map(|vert| {
                let removal = if key < vert.key {
                    if !(vert.has_red_left() || vert.has_red_left_left()) {
                        vert.restruct_left();
                    }
                    Self::remove_vertex(&mut vert.left, key)
                } else {
                    if vert.has_red_left() {
                        vert.rot_right();
                    }

                    if key == vert.key && vert.right.is_none() {
                        return (&mut *opt_vert).take();
                    }

                    if !(vert.has_red_right() || vert.has_red_right_left()) {
                        vert.restruct_right();
                    }

                    if key == vert.key {
                        vert.swap_successor();
                        Some(Self::pop_min_vertex(&mut vert.right))
                    } else {
                        Self::remove_vertex(&mut vert.right, key)
                    }
                };
                vert.rebalance();
                removal
            })?
        }
    }
}

// 迭代器方法
impl<'a, V> Vertex<'a, V> {
    pub(super) fn preorder(&'a self) -> PreorderIter<'a, V> {
        PreorderIter::new(self)
    }
}
