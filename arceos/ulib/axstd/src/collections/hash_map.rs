extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;

const DEFAULT_CAPACITY: usize = 55000;
pub struct HashMap<K, V> {
    buckets: Vec<Option<(K, V)>>,
}

impl<K: PartialEq + Clone, V: Clone> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: vec![None; DEFAULT_CAPACITY],
        }
    }

    fn hash(&self, k: &K) -> usize {
        unsafe {*(k as *const _ as *const usize)}
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut idx = self.hash(&key) % self.buckets.len();

        loop {
            match &self.buckets[idx] {
                Some((exsitint_key, _)) if exsitint_key == &key => {
                    self.buckets[idx] = Some((key, value));
                    return;
                }
                None => {
                    self.buckets[idx] = Some((key, value));
                    return;
                }
                _ => {
                    idx = (idx + 1) % self.buckets.len();
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut idx = self.hash(&key) % self.buckets.len();

        loop {
            match &self.buckets[idx] {
                Some((exsitint_key, value)) if *exsitint_key == *key => return Some(value),
                None => return None,
                _ => idx = (idx + 1) % self.buckets.len(),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.buckets.iter().filter_map(|item| item.as_ref())
    }
}
