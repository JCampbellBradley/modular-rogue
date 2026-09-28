
use std::collections::HashMap;

use serde::Deserialize;

use crate::{blueprints::blueprint::Blueprint, entities::entity::Entity, errors::blueprint_error::BlueprintError::{self}, glue::mod_registry::ModRegistry};

#[derive(Debug, Deserialize)]
pub struct EntityBlueprint {
    components: HashMap<String, ron::Value>,
}

impl Blueprint<Entity> for EntityBlueprint {
    fn build(self, reg: &ModRegistry) -> Result<Entity, BlueprintError> {
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