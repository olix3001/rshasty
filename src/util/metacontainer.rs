use std::{any::{TypeId, Any}, collections::HashMap, fmt::Debug, cell::RefCell, rc::Rc};

/// Container for all metadata (used for additional information in AST)
#[derive(Clone)]
pub struct MetaContainer {
    pub meta: Rc<RefCell<HashMap<TypeId, Box<dyn Any>>>>
}

impl Debug for MetaContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaContainer")
            .field("meta", &self.meta)
            .finish()
    }
}

impl MetaContainer {
    pub fn new() -> Self {
        Self {
            meta: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn get<'a, T: 'static>(&'a self) -> Option<Rc<T>> {
        self.meta.borrow().get(&TypeId::of::<T>())
            .and_then(|x| x.downcast_ref::<Rc<T>>().and_then(|x| Some(Rc::clone(x))))
    }

    pub fn insert<T: 'static>(&mut self, value: T) {
        self.meta.borrow_mut().insert(TypeId::of::<T>(), Box::new(Rc::new(value)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metacontainer() {
        let mut meta = MetaContainer::new();
        meta.insert(1);
        assert_eq!(*meta.get::<i32>().unwrap(), 1);
    }
}