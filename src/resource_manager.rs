use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::camera::Camera;

pub struct ResourceManager {
    resources: HashMap<u64, Box<dyn Any>>,
    camera: Box<Camera>,

}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            camera: Box::new(Camera::new())
        }
    }

    pub fn add_resource<T: 'static>(&mut self, key: &str, resource: T) {
        let hash = Self::hash_string(key);
        if self.resources.contains_key(&hash) {
            println!("\"{}\" was already found in resource map, replacing", key);
            self.resources.remove(&hash);
        }
        self.resources.insert(hash, Box::new(resource));
    }

    pub fn get_resource<T: 'static>(&self, key: &str) -> Option<&Box<T>> {
        let hash = Self::hash_string(key);
        if let Some(boxed_any) = self.resources.get(&hash) {
            let type_name = std::any::type_name_of_val(&*boxed_any);
            match boxed_any.downcast_ref::<Box<T>>() {
                Some(value) => Some(value),
                None => {
                    println!("Failed to downcast: expected type {}, actual type {}", 
                        std::any::type_name::<T>(), 
                        type_name);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_resource_mut<T: 'static>(&mut self, key: &str) -> Option<&mut Box<T>> {
        let hash = Self::hash_string(key);
        if let Some(boxed_any) = self.resources.get_mut(&hash) {
            let type_name = std::any::type_name_of_val(&*boxed_any);
            match boxed_any.downcast_mut::<Box<T>>() {
                Some(value) => Some(value),
                None => {
                    println!("Failed to downcast: expected type {}, actual type {}", 
                        std::any::type_name::<T>(), 
                        type_name);
                    None
                }
            }
        } else {
            None
        }
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
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}