#![doc = include_str!("../README.md")]

use std::{borrow::Borrow, collections::HashMap, hash::Hash};

#[derive(Clone, Debug)]
pub struct TrainMap<'a, K, V> {
    map: HashMap<K, V>,
    parent: Option<&'a TrainMap<'a, K, V>>,
}

impl<'a, K: Eq + Hash, V> TrainMap<'a, K, V> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
            parent: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            parent: None,
        }
    }

    pub fn get<Q: Eq + Hash + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        if let Some(value) = self.map.get(key) {
            Some(value)
        } else if let Some(parent) = self.parent {
            parent.get(key)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn fork(&'a self) -> Self {
        Self {
            map: Default::default(),
            parent: Some(self),
        }
    }
}

impl<'a, K: Eq + Hash, V> Default for TrainMap<'a, K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Eq + Hash, V> Extend<(K, V)> for TrainMap<'a, K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iterator: T) {
        self.map.extend(iterator)
    }
}
