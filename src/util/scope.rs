use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope<T> {
    pub variables: HashMap<String, T>,
}

impl<T: Clone> Scope<T> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, value: T) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<T> {
        self.variables.get(name).cloned()
    }

    pub fn remove(&mut self, name: &str) -> Option<T> {
        self.variables.remove(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.variables.iter()
    }

    pub fn merge(&mut self, other: &Self) {
        for (name, value) in other.iter() {
            self.add(name, value.clone());
        }
    }

    pub fn child(&self) -> Self {
        Self {
            variables: self.variables.clone(),
        }
    }
}