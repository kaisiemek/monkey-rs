use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::object::Object;

#[derive(Debug)]
pub struct Environment {
    identifiers: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            identifiers: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.identifiers.get(name) {
            Some(object) => Some(object.clone()),
            None => self
                .outer
                .as_ref()
                .and_then(|outer| outer.borrow().get(name)),
        }
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.identifiers.insert(name.to_string(), value);
    }
}
