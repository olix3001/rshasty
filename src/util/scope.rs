use std::{collections::HashMap, cell::RefCell, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
// Simple scope implementation. Used for storing variables.
// Can be cheaply cloned as it uses Rc<RefCell<HashMap>>.
pub struct GenericScope<T> {
    pub variables: Rc<RefCell<HashMap<String, T>>>,
    pub parent: Option<Rc<GenericScope<T>>>
}

pub struct Scope<T>(Rc<GenericScope<T>>);

impl<T: Clone> Scope<T> {
    pub fn new() -> Self {
        Self(Rc::new(GenericScope {
            variables: Rc::new(RefCell::new(HashMap::new())),
            parent: None
        }))
    }

    pub fn child(&self) -> Self {
        Self(Rc::new(GenericScope {
            variables: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Rc::clone(&self.0))
        }))
    }

    pub fn get(&self, name: &str) -> Option<T> {
        let mut scope = Some(&self.0);

        while let Some(current) = scope {
            if let Some(value) = current.variables.borrow().get(name) {
                return Some(value.clone());
            }

            scope = current.parent.as_ref();
        }

        None
    }

    pub fn insert(&self, name: &str, value: T) {
        self.0.variables.borrow_mut().insert(name.to_string(), value);
    }

    pub fn set(&self, name: &str, value: T) {
        let mut scope = Some(&self.0);

        while let Some(current) = scope {
            if let Some(_) = current.variables.borrow().get(name) {
                current.variables.borrow_mut().insert(name.to_string(), value);
                return;
            }

            scope = current.parent.as_ref();
        }

        self.0.variables.borrow_mut().insert(name.to_string(), value);
    }

    pub fn parent(&self) -> Option<Self> {
        if let Some(parent) = &self.0.parent {
            Some(Self(Rc::clone(parent)))
        } else {
            None
        }
    }

}