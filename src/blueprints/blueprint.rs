use crate::{errors::blueprint_error::BlueprintError, glue::mod_registry::ModRegistry};

pub trait Blueprint<T> {
    fn build(self, reg: &ModRegistry) -> Result<T, BlueprintError>;
}