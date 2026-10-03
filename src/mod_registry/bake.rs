use crate::{errors::bake_error::BakeError, mod_registry::registry::ModRegistry};

pub trait Bake<T> {
    fn bake(self, mod_registry: &ModRegistry) -> Result<T, BakeError>;
}

pub trait Unbake<T> {
    fn unbake(target: &T, mod_registry: &ModRegistry) -> Result<Self, BakeError> where Self : Sized;
}