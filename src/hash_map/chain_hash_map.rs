use super::Node;
use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[allow(dead_code)]
struct ChainHashMap<'a, K: Eq + Hash, V> {
    base: Vec<LinkedList<Node<K, V>>>,
    len: usize,
    dvs: u64,
    marker: PhantomData<&'a V>,
}

impl<'a, K, V> ChainHashMap<'a, K, V>
where
    K: Eq + Hash,
{
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % self.dvs
    }
}

#[allow(dead_code)]
impl<'a, K, V> ChainHashMap<'a, K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        let dvs: u64 = 97;
        let mut base = Vec::with_capacity(dvs as usize);
        (0..dvs).for_each(|_| base.push(LinkedList::new()));

        Self {
            dvs,
            base,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let hash_code = self.hash(&key);

        match self.base[hash_code as usize]
            .iter_mut()
            .find(|node| node.hash_code == hash_code)
        {
            Some(node) => Some(node.replace(val)),
            None => {
                self.base[hash_code as usize].push_back(Node::new(key, val, hash_code));
                self.len += 1;
                None
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash_code = self.hash(key);

        self.base[hash_code as usize]
            .iter()
            .find(|node| node.hash_code == hash_code)
            .map(|node| &node.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let hash_code = self.hash(key);

        self.base[hash_code as usize]
            .drain_filter(|node| node.hash_code == hash_code)
            .last()
            .map(|node| {
                self.len -= 1;
                node.into_value()
            })
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.base[self.hash(key) as usize]
            .iter()
            .any(|node| &node.key == key)
    }
}

#[cfg(test)]
mod tests {
    use super::ChainHashMap;
    #[test]
    fn crud_chain_hashmap() {
        let mut map: ChainHashMap<'_, u64, &str> = ChainHashMap::new();

        assert_eq!(map.insert(15, "Mike"), None);
        assert_eq!(map.remove(&15), Some("Mike"));
        assert_eq!(map.contains_key(&15), false);
    }
}
