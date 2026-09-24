use std::collections::HashMap;


pub type RegistryID = u64;

#[derive(Default)]
pub struct Registry<T: Registerable> {
    map: HashMap<RegistryID, T>,
    last_id: RegistryID
}

impl<T: Registerable> Registry<T> {
    pub fn register(&mut self, mut item: T) -> RegistryID {
        self.last_id += 1;
        item.set_id(self.last_id);
        self.map.insert(self.last_id, item);
        self.last_id
    }

    pub fn unregister(&mut self, id: RegistryID) -> Option<T> {
        self.map.remove(&id)
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
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::worlds::test_util::TestRegisterable;

    use super::*;

    #[test]
    fn id_increments() {
        let mut reg: Registry<TestRegisterable> = Registry::default();

        let id1 = reg.register(TestRegisterable::default());
        let id2 = reg.register(TestRegisterable::default());


        assert_eq!(id1+1, id2);
    }

    #[test]
    fn get() {
        let mut reg: Registry<TestRegisterable> = Registry::default();

        let id = reg.register(TestRegisterable::default());
        
        assert!(reg.get(id).is_some())

    }

    #[test]
    fn get_mut() {
        let mut reg: Registry<TestRegisterable> = Registry::default();

        let id = reg.register(TestRegisterable::default());
        
        assert!(reg.get_mut(id).is_some())
    }

    #[test]
    fn keys() {
        let mut reg: Registry<TestRegisterable> = Registry::default();

        let id1 = reg.register(TestRegisterable::default());
        let id2 = reg.register(TestRegisterable::default());
        let id3 = reg.register(TestRegisterable::default());

        reg.unregister(id2);

        let keys: Vec<RegistryID> = reg.keys().collect();

        assert!(keys.len() == 2);
        assert!(keys.contains(&id1));
        assert!(!keys.contains(&id2));
        assert!(keys.contains(&id3));
    }
}