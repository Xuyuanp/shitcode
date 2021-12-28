use std::mem::replace;

pub struct StupidMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K: Eq, V> StupidMap<K, V> {
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
}
