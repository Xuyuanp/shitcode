use std::mem::replace;

pub struct Iter<'a, K, V> {
    inner: &'a StupidMap<K, V>,
    idx: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.inner.len() {
            let ent = &self.inner.entries[self.idx];
            self.idx += 1;
            Some((&ent.0, &ent.1))
        } else {
            None
        }
    }
}

pub struct IterMut<'a, K, V> {
    inner: &'a mut StupidMap<K, V>,
    idx: usize,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.inner.len() {
            let idx = self.idx;
            self.idx += 1;
            let ptr = self.inner.entries.as_mut_ptr();
            unsafe {
                let ent = &mut *ptr.add(idx);
                Some((&ent.0, &mut ent.1))
            }
        } else {
            None
        }
    }
}

pub struct StupidMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> StupidMap<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            inner: self,
            idx: 0,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            inner: self,
            idx: 0,
        }
    }
}

impl<K: Eq, V> StupidMap<K, V> {
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if let Some(ent) = self.entries.iter_mut().find(|ent| ent.0 == key) {
            let old = replace(&mut ent.1, val);
            return Some(old);
        }
        self.entries.push((key, val));
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .iter()
            .find(|ent| &ent.0 == key)
            .and_then(|ent| Some(&ent.1))
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.entries
            .iter_mut()
            .find(|ent| &ent.0 == key)
            .and_then(|ent| Some(&mut ent.1))
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.entries
            .iter()
            .find(|ent| &ent.0 == key)
            .and_then(|ent| Some((&ent.0, &ent.1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = StupidMap::new();

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.insert(3, "three"), None);
        assert_eq!(map.insert(1, "One"), Some("one"));
    }

    #[test]
    fn len() {
        let mut map = StupidMap::new();

        assert_eq!(map.len(), 0);

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.len(), 2);
        assert_eq!(map.insert(3, "three"), None);
        assert_eq!(map.len(), 3);
        assert_eq!(map.insert(1, "One"), Some("one"));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn clear() {
        let mut map = StupidMap::new();

        assert_eq!(map.len(), 0);

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.len(), 2);
        assert_eq!(map.insert(3, "three"), None);
        assert_eq!(map.len(), 3);
        assert_eq!(map.insert(1, "One"), Some("one"));
        assert_eq!(map.len(), 3);

        map.clear();

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn get() {
        let mut map = StupidMap::new();

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.insert(3, "three"), None);
        assert_eq!(map.insert(1, "One"), Some("one"));

        assert_eq!(map.get(&1), Some(&"One"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
        assert_eq!(map.get(&4), None);
    }

    #[test]
    fn get_mut() {
        let mut map = StupidMap::new();

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.insert(3, "three"), None);
        assert_eq!(map.insert(1, "One"), Some("one"));

        let val = map.get_mut(&2);
        assert_eq!(val, Some(&mut "two"));
        val.map(|v| *v = "Two");

        assert_eq!(map.get(&2), Some(&"Two"));
    }

    #[test]
    fn get_key_value() {
        let mut map = StupidMap::new();

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.insert(3, "three"), None);
        assert_eq!(map.insert(1, "One"), Some("one"));

        assert_eq!(map.get_key_value(&1), Some((&1, &"One")));
        assert_eq!(map.get_key_value(&2), Some((&2, &"two")));
        assert_eq!(map.get_key_value(&3), Some((&3, &"three")));
        assert_eq!(map.get_key_value(&4), None);
    }

    #[test]
    fn iter() {
        let mut map = StupidMap::new();

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.insert(3, "three"), None);

        let mut iter = map.iter();

        assert_eq!(iter.next(), Some((&1, &"one")));
        assert_eq!(iter.next(), Some((&2, &"two")));
        assert_eq!(iter.next(), Some((&3, &"three")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut map = StupidMap::new();

        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "two"), None);
        assert_eq!(map.insert(3, "three"), None);

        let first = map.iter_mut().next();

        assert_eq!(first, Some((&1, &mut "one")));

        let first = first.unwrap();

        *first.1 = "One";

        assert_eq!(map.get(&1), Some(&"One"));

        let mut iter = map.iter_mut();
        assert_eq!(iter.next(), Some((&1, &mut "One")));
        assert_eq!(iter.next(), Some((&2, &mut "two")));
        assert_eq!(iter.next(), Some((&3, &mut "three")));
        assert_eq!(iter.next(), None);
    }
}
