use std::hash::Hash;
use std::mem;

mod chain_hash_map;
mod linear_hash_map;

#[allow(dead_code)]
struct Node<K: Eq + Hash, V> {
    key: K,
    value: V,
    hash_code: u64,
}

impl<K, V> Node<K, V>
where
    K: Eq + Hash,
{
    fn new(key: K, value: V, hash_code: u64) -> Self {
        Self {
            key,
            value,
            hash_code,
        }
    }

    fn replace(&mut self, val: V) -> V {
        mem::replace(&mut self.value, val)
    }

    fn into_value(self) -> V {
        self.value
    }
}
