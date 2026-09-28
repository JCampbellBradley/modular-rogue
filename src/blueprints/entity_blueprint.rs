
use std::collections::HashMap;

use serde::Deserialize;

use crate::{entities::entity::Entity, errors::{bake_error::BakeError}, glue::{bake::Bake, mod_registry::ModRegistry}};

#[derive(Debug, Deserialize)]
pub struct EntityBlueprint {
    components: HashMap<String, ron::Value>,
}

impl Bake<Entity> for EntityBlueprint {
    fn bake(self, reg: &ModRegistry) -> Result<Entity, BakeError> {
        let mut entity = Entity::new();
        for (tag, value) in self.components {
            let component = reg.deserialize_component(tag, value)?;
            entity.attach(component)?;
        }
        Ok(entity)
    }
}

#[cfg(test)]
mod tests {
    
}