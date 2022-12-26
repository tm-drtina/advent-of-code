use std::cell::RefCell;
use std::collections::HashMap;

type CacheInitializer<K, V> = Box<dyn Fn(&K, &Cache<K, V>) -> V>;

pub struct Cache<K, V> {
    memory: RefCell<HashMap<K, V>>,
    initializer: CacheInitializer<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    pub fn new(initializer: CacheInitializer<K, V>) -> Self {
        Self {
            memory: RefCell::default(),
            initializer,
        }
    }

    pub fn get_or_compute(&self, key: K) -> V {
        if let Some(value) = self.memory.borrow().get(&key) {
            return value.clone();
        }
        let value = (self.initializer)(&key, self);
        self.memory.borrow_mut().insert(key, value.clone());
        value
    }
}
