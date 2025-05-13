extern crate alloc;
extern crate axalloc;

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;


const DEFAULT_CAPACITY: usize = 55000;
pub struct HashMap<String, V> {
    buckets: Vec<Option<(String, V)>>,
}

impl<V: Clone> HashMap<String, V> {
    pub fn new() -> Self {
        Self {
            buckets: vec![None; DEFAULT_CAPACITY],
        }
    }

    fn hash(&self, k: &String) -> usize {
        let mut hash = 0usize;
        for byte in k.bytes() {
            hash = hash.wrapping_shl(5) ^ hash.wrapping_shr(2) ^ (byte as usize);
        }
        hash
    }

    pub fn insert(&mut self, key: String, value: V) {
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

    pub fn get(&self, key: &String) -> Option<&V> {
        let mut idx = self.hash(&key) % self.buckets.len();

        loop {
            match &self.buckets[idx] {
                Some((exsitint_key, value)) if *exsitint_key == *key => return Some(value),
                None => return None,
                _ => idx = (idx + 1) % self.buckets.len(),
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, V)> {
        self.buckets.iter().filter_map(|item| item.as_ref())
    }
}
