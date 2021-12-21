use super::Node;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

#[allow(dead_code)]
pub struct LinearHashMap<K: Eq + Hash, V> {
    base: Vec<Option<Node<K, V>>>,
    len: usize,
    capacity: u64,
}

impl<K, V> LinearHashMap<K, V>
where
    K: Eq + Hash,
{
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % self.capacity
    }

    fn find_none(&self, mut start: usize) -> usize {
        while let Some(_) = self.base[start] {
            start = (start + 1) % self.capacity as usize;
        }

        start
    }

    fn resize(&mut self, cap: u64) {
        // 只有两种情况：乘二 或 除二
        self.capacity = cap;

        // 创建新数组并用 None 填充
        let mut re_base = Vec::with_capacity(self.capacity as usize);
        (0..self.capacity).for_each(|_| re_base.push(None));

        // 取出旧数组的节点
        let old_base = mem::replace(&mut self.base, re_base);
        for opt_node in old_base {
            // 操作节点
            if let Some(mut node) = opt_node {
                // 根据新上限设置节点的哈希值
                node.hash_code = self.hash(&node.key);

                // 找到空位
                let i = self.find_none(node.hash_code as usize);

                // 储存新节点
                self.base[i] = Some(node);
            }
        }
    }
}

#[allow(dead_code)]
impl<K, V> LinearHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        let capacity: u64 = 16;
        let mut base = Vec::with_capacity(capacity as usize);
        (0..16).for_each(|_| base.push(None));
        Self {
            base,
            len: 0,
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.len as u64 >= self.capacity / 2 {
            self.resize(self.capacity * 2);
        }

        let hash_code = self.hash(&key);
        let node = Node::new(key, val, hash_code);
        let mut i = hash_code as usize;

        while let Some(old_node) = &mut self.base[i] {
            if old_node.key == node.key {
                return self.base[i].replace(node).map(|old| old.into_value());
            }

            i += (i + 1) % self.capacity as usize;
        }

        self.len += 1;
        self.base[i] = Some(node);
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut i = self.hash(key) as usize;

        while let Some(node) = &self.base[i] {
            if &node.key == key {
                break;
            }
            i = (i + 1) % self.capacity as usize;
        }

        self.base[i].take().map_or(None, |node| {
            // 从删除位的下一位开始，
            i = (i + 1) % self.capacity as usize;
            // 遍历删除节点所在键簇。
            while let Some(_) = &self.base[i] {
                // 寻找空位
                let new_idx = self.find_none(i);

                // 无论是否找到空位，都尝试交换
                self.base.swap(i, new_idx);

                // 步进键簇
                i = (i + 1) % self.capacity as usize;
            }

            self.len -= 1;

            if self.len > 0 && self.len == self.capacity as usize / 8 {
                self.resize(self.capacity / 2);
            }

            Some(node.into_value())
        })
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let mut i = self.hash(key) as usize;

        loop {
            match &self.base[i] {
                Some(node) if &node.key == key => return true,
                None => return false,
                Some(_) => i = (i + 1) % self.capacity as usize,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinearHashMap;

    #[test]
    fn crud_linear_hashmap() {
        let mut map: LinearHashMap<u32, &str> = LinearHashMap::new();
        assert_eq!(map.insert(15, "Mike"), None);
        assert_eq!(map.remove(&15), Some("Mike"));
        assert_eq!(map.contains_key(&15), false);
    }
}
