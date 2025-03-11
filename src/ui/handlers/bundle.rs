use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Bundle {
    data: HashMap<String, Box<dyn Any>>,
}
impl Bundle {

    pub fn new() -> Self {
        Bundle {
            data: HashMap::new(),
        }
    }

    pub fn put<T: Any>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), Box::new(value));
    }

    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        self.data.get(key)?.downcast_ref::<T>()
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}
