use thiserror::Error;

use crate::worlds::registry::RegistryID;


#[derive(Error, Debug)]
pub enum IncRegistryError {
    #[error("Could not register {item} because it was already had an IncrementalRegistry id.")]
    Reregistration {
        item: String
    },
    #[error("Could not manually register item with id {id:?}, as it was not an increment of the last id {last_id:?}.")]
    BadManualID {
        id: RegistryID,
        last_id: RegistryID,
    }
}