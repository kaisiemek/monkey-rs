use std::collections::HashMap;

use super::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    identifiers: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.identifiers.get(name)
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.identifiers.insert(name.to_string(), value);
    }
}
