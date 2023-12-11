use std::{any::{TypeId, Any}, collections::HashMap};

/// Container for all metadata (used for additional information in AST)
pub struct MetaContainer {
    pub meta: HashMap<TypeId, Box<dyn Any>>
}

impl MetaContainer {
    pub fn new() -> Self {
        Self {
            meta: HashMap::new()
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.meta.get(&TypeId::of::<T>()).and_then(|x| x.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.meta.get_mut(&TypeId::of::<T>()).and_then(|x| x.downcast_mut::<T>())
    }

    pub fn insert<T: 'static>(&mut self, value: T) {
        self.meta.insert(TypeId::of::<T>(), Box::new(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metacontainer() {
        let mut meta = MetaContainer::new();
        meta.insert(1);
        assert_eq!(meta.get::<i32>().unwrap(), &1);
        assert_eq!(meta.get_mut::<i32>().unwrap(), &mut 1);
    }
}