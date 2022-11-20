use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type CacheInitializer<K, V> = Box<dyn Fn(&K, &Cache<K, V>) -> V + Sync>;

pub struct Cache<K, V> {
    memory: Arc<Mutex<HashMap<K, V>>>,
    initializer: CacheInitializer<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    pub fn new(initializer: CacheInitializer<K, V>) -> Self {
        Self {
            memory: Default::default(),
            initializer,
        }
    }

    pub fn get_or_compute(&self, key: K) -> V {
        let guard = self.memory.lock().unwrap();
        if let Some(value) = guard.get(&key) {
            return value.clone();
        }
        drop(guard);
        let value = (self.initializer)(&key, self);
        self.put(key, value.clone());
        value
    }

    fn put(&self, key: K, value: V) {
        let mut mem = self.memory.lock().unwrap();
        mem.insert(key, value);
    }
}
