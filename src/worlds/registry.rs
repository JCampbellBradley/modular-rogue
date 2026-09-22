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

    pub fn keys(&self) -> Vec<RegistryID> {
        self.map.keys().cloned().collect()
    }
}

pub trait Registerable {
    fn get_id(&self) -> Option<RegistryID>;
    fn set_id(&mut self, id: RegistryID);
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Foo {
        id: Option<RegistryID>
    }

    impl Registerable for Foo {
        fn get_id(&self) -> Option<RegistryID> {
            self.id
        }

        fn set_id(&mut self, id: RegistryID) {
            self.id = Some(id);
        }
    }

    #[test]
    fn id_increments() {
        let mut reg: Registry<Foo> = Registry::default();

        assert_eq!(reg.register(Foo::default()), 1);
        assert_eq!(reg.register(Foo::default()), 2);

    }
}