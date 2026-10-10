use std::{collections::HashMap, fmt::Debug};

use crate::errors::inc_registry_error::IncRegistryError;


pub type RegistryID = u64;

#[derive(Default, Debug)]
pub struct IncRegistry<T: Registerable> {
    map: HashMap<RegistryID, T>,
    last_id: RegistryID
}

impl<T: Registerable + Default + Debug> IncRegistry<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Registerable + Debug> IncRegistry<T> {
    pub fn register(&mut self, mut item: T) -> Result<RegistryID, IncRegistryError> {
        if let Some(_) = item.get_id() {
            return Err(IncRegistryError::Reregistration { item: format!("{item:?}") })
        }
        
        self.last_id += 1;
        item.set_id(self.last_id);
        self.map.insert(self.last_id, item);
        Ok(self.last_id)
    }
    
    ///For use with `next_id`, when an id is needed before registration is safe.
    pub fn register_with(&mut self, mut item: T, id: RegistryID) -> Result<RegistryID, IncRegistryError> {
        if id == self.last_id + 1 {
            self.last_id = id;
            item.set_id(id);
            self.map.insert(id, item);
            Ok(id)
        } else {
            Err(IncRegistryError::BadManualID { id, last_id: self.last_id })
        }
        
    }

    pub fn next_id(&self) -> RegistryID {
        self.last_id + 1
    }

    pub fn unregister(&mut self, id: RegistryID) -> Option<T> {
        if let Some(mut item) = self.map.remove(&id) {
            item.remove_id();
            self.last_id -= 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn get(&self, id: RegistryID) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: RegistryID) -> Option<&mut T> {
        self.map.get_mut(&id)
    }

    pub fn keys(&self) -> impl Iterator<Item=RegistryID> {
        Box::new(self.map.keys().cloned())
    }
}

pub trait Registerable {
    fn get_id(&self) -> Option<RegistryID>;
    fn set_id(&mut self, id: RegistryID);
    fn remove_id(&mut self);
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::worlds::test_util::TestRegisterable;

    use super::*;

    #[test]
    fn id_increments() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let id1 = reg.register(TestRegisterable::default()).unwrap();
        let id2 = reg.register(TestRegisterable::default()).unwrap();


        assert_eq!(id1+1, id2);
    }

    #[test]
    fn get() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let id = reg.register(TestRegisterable::default()).unwrap();
        
        assert!(reg.get(id).is_some())

    }

    #[test]
    fn get_mut() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let id = reg.register(TestRegisterable::default()).unwrap();
        
        assert!(reg.get_mut(id).is_some())
    }

    #[test]
    fn keys() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let id1 = reg.register(TestRegisterable::default()).unwrap();
        let id2 = reg.register(TestRegisterable::default()).unwrap();
        let id3 = reg.register(TestRegisterable::default()).unwrap();

        reg.unregister(id2);

        let keys: Vec<RegistryID> = reg.keys().collect();

        assert!(keys.len() == 2);
        assert!(keys.contains(&id1));
        assert!(!keys.contains(&id2));
        assert!(keys.contains(&id3));
    }

    #[test]
    fn double_register() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let registerable = TestRegisterable::default();

        let id = reg.register(registerable).unwrap();

        let registerable = reg.get(id).unwrap().clone();

        let err = reg.register(registerable).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn deregister_register() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let id = reg.register(TestRegisterable::default()).unwrap();
        let registerable = reg.unregister(id).unwrap();
        reg.register(registerable).unwrap();
    }

    #[test]
    fn register_with() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        reg.register(TestRegisterable::default()).unwrap();
        let next_id = reg.next_id();
        reg.register_with(TestRegisterable::default(), next_id).unwrap();
    }

    #[test]
    fn bad_register_with() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        let id = reg.register(TestRegisterable::default()).unwrap();
        let err = reg.register_with(TestRegisterable::default(), id).unwrap_err();

        println!("{err}");
    }

    #[test]
    fn unregister_none() {
        let mut reg: IncRegistry<TestRegisterable> = IncRegistry::default();

        assert!(reg.unregister(0).is_none());
    }
}