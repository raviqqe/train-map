#![doc = include_str!("../README.md")]

use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
};

#[derive(Clone, Debug)]
pub struct TrainMap<'a, K, V, S = RandomState> {
    map: HashMap<K, V, S>,
    parent: Option<&'a TrainMap<'a, K, V, S>>,
}

impl<'a, K: Eq + Hash, V> TrainMap<'a, K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, Default::default())
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> TrainMap<'a, K, V, S> {
    pub fn with_hasher(hasher: S) -> Self {
        Self {
            map: HashMap::with_hasher(hasher),
            parent: None,
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, hasher),
            parent: None,
        }
    }

    pub fn hasher(&self) -> &S {
        self.map.hasher()
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

    pub fn fork(&'a self) -> Self
    where
        S: Clone,
    {
        Self {
            map: HashMap::with_hasher(self.map.hasher().clone()),
            parent: Some(self),
        }
    }
}

impl<'a, K: Eq + Hash, V, S: Default> Default for TrainMap<'a, K, V, S> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            parent: None,
        }
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> Extend<(K, V)> for TrainMap<'a, K, V, S> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iterator: T) {
        self.map.extend(iterator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{BuildHasherDefault, DefaultHasher};

    #[derive(Clone, Debug, Default, PartialEq)]
    struct TaggedState(usize);

    impl BuildHasher for TaggedState {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    #[test]
    fn create_with_hasher() {
        let mut map = TrainMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

        map.insert("foo", 42);

        assert_eq!(map.get("foo"), Some(&42));
    }

    #[test]
    fn create_with_capacity_and_hasher() {
        let mut map =
            TrainMap::with_capacity_and_hasher(1, BuildHasherDefault::<DefaultHasher>::default());

        map.insert("foo", 42);

        assert_eq!(map.get("foo"), Some(&42));
    }

    #[test]
    fn create_default_with_hasher() {
        let mut map = TrainMap::<_, _, BuildHasherDefault<DefaultHasher>>::default();

        map.insert("foo", 42);

        assert_eq!(map.get("foo"), Some(&42));
    }

    #[test]
    fn get_hasher() {
        assert_eq!(
            TrainMap::<(), (), _>::with_hasher(TaggedState(42)).hasher(),
            &TaggedState(42)
        );
    }

    #[test]
    fn fork_with_hasher() {
        let mut parent = TrainMap::with_hasher(TaggedState(42));

        parent.insert("foo", 1);

        let mut child = parent.fork();

        child.insert("bar", 2);

        assert_eq!(child.hasher(), &TaggedState(42));
        assert_eq!(child.get("foo"), Some(&1));
        assert_eq!(child.get("bar"), Some(&2));
        assert_eq!(parent.get("bar"), None);
    }
}
