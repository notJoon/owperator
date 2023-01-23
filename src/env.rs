use std::{rc::Rc, cell::RefCell, collections::HashMap};

use crate::parser::Object;

#[derive(Debug, PartialEq)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    vars: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            parent: None,
            vars: HashMap::new(),
        }
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Self {
        Environment {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(o) => Some(o.clone()),
            None => self.parent
                        .as_ref()
                        .and_then(|o| o.borrow().get(name)),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}