pub trait Map<K, V> {
    #[must_use]
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K: Eq, V> Map<K, V> for (K, V) {
    fn get(&self, key: &K) -> Option<&V> {
        let (k, v) = self;
        key.eq(k).then(|| v)
    }
}

impl<'a, F, K: Eq, V: 'a> Map<K, V> for &'a F
where
    F: Fn(&K) -> Option<&'a V>,
{
    fn get(&self, key: &K) -> Option<&V> {
        self(key)
    }
}

impl<K: Eq, V, const N: usize> Map<K, V> for [(K, V); N] {
    fn get(&self, key: &K) -> Option<&V> {
        self.iter().find_map(|(k, v)| key.eq(k).then(|| v))
    }
}
