use std::{collections::HashMap, hash::Hash};

pub(crate) struct OptMap<K, V> {
    data: Option<HashMap<K, V>>,
}
impl<K: Eq + Hash, V> OptMap<K, V> {
    pub(crate) fn new() -> Self {
        Self { data: None }
    }
    // Query methods
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.data.as_ref().map(|v| v.get(key)).unwrap_or(None)
    }
    // Modification methods
    pub(crate) fn insert(&mut self, key: K, value: V) {
        match self.data.as_mut() {
            Some(map) => {
                map.insert(key, value);
                ()
            }
            None => {
                let mut map = HashMap::with_capacity(1);
                map.insert(key, value);
                self.data = Some(map);
            }
        };
    }
    pub(crate) fn remove(&mut self, key: &K) {
        match self.data.as_mut() {
            Some(map) => {
                map.remove(key);
                if map.is_empty() {
                    self.data = None;
                };
            }
            None => (),
        };
    }
}
