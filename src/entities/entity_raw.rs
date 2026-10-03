use serde::{Deserialize, Serialize};

use crate::{entities::entity::Entity, errors::bake_error::BakeError, mod_registry::{bake::{Bake, Unbake}, registry::ModRegistry}, worlds::{position::Position, registry::{Registerable, RegistryID}}};

#[derive(Serialize, Deserialize, Default)]
pub struct EntityRaw {
    id: Option<RegistryID>,
    components: Vec<(String, String)>,
    zone_position: Option<Position>,
    position: Option<Position>,
}

impl Bake<Entity> for EntityRaw {
    fn bake(self, mod_registry: &ModRegistry) -> Result<Entity, BakeError> {
        let mut ent = Entity::new();
        if let Some(id) = self.id { ent.set_id(id); }
        ent.zone_position = self.zone_position;
        ent.position = self.position;

        for (tag, text) in self.components {
            let value = ron::from_str(&text)?;
            let component = mod_registry.deserialize_component(tag, value)?;
            ent.attach(component)?;
        }

        Ok(ent)
    }
}

impl Unbake<Entity> for EntityRaw {
    fn unbake(target: &Entity, mod_registry: &ModRegistry) -> Result<EntityRaw, BakeError> {
        let mut raw = EntityRaw {
            id: target.get_id(),
            zone_position: target.zone_position,
            position: target.position,
            ..Default::default()
        };

        for comp in target.components() {
            let (tag, value) = mod_registry.serialize_component(comp)?;
            raw.components.push((tag, value));
        }

        Ok(raw)
    }
}