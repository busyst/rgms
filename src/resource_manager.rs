use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct ResourceManager {
    resources: HashMap<u64, Box<dyn Any>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add_resource(&mut self, key: &str, resource: Box<dyn Any>) {
        let hash = Self::hash_string(key);
        self.resources.insert(hash, resource);
    }

    pub fn get_resource<T: 'static>(&self, key: &str) -> Option<&T> {
        let hash = Self::hash_string(key);
        self.resources.get(&hash)
            .and_then(|boxed_any| boxed_any.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: 'static>(&mut self, key: &str) -> Option<&mut T> {
        let hash = Self::hash_string(key);
        self.resources.get_mut(&hash)
            .and_then(|boxed_any| boxed_any.downcast_mut::<T>())
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }

    // Private helper
    fn hash_string(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
}