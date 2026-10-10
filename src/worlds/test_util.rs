use crate::worlds::registry::{Registerable, RegistryID};


#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TestRegisterable {
    id: Option<RegistryID>
}

impl Registerable for TestRegisterable {
    fn get_id(&self) -> Option<RegistryID> {
        self.id
    }

    fn set_id(&mut self, id: RegistryID) {
        self.id = Some(id);
    }

    fn remove_id(&mut self) {
        self.id = None;
    }
}