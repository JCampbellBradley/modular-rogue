use serde::{Deserialize, Serialize};

use crate::{entities::entity::Entity, errors::{bake_error::BakeError, expect_messages::LOGICALLY_UNREACHABLE}, mod_registry::{bake::{Bake, Unbake}, registry::ModRegistry}, worlds::{position::Position, registry::{Registerable, RegistryID}}};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EntityRaw {
    id: Option<RegistryID>,
    components: Vec<(String, Box<ron::value::RawValue>)>,
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
            let component = mod_registry.deserialize_component(tag, text.as_ref())?;
            ent.attach(component).expect(LOGICALLY_UNREACHABLE);
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

#[cfg(test)]
mod tests {
    use ron::from_str;

use crate::{components::test_util::{TestStatefulComponent, TestStatelessComponent}, mod_registry::test_util::reg_with_test_components, worlds::registry::Registry};

use super::*;

    #[test]
    fn unbake() {
        let reg = reg_with_test_components();

        let mut ent = Entity::new();
        ent.require::<TestStatelessComponent>();
        let a = ent.require::<TestStatefulComponent>();

        a.attr1 = 1;

        let raw = EntityRaw::unbake(&ent, &reg).unwrap();

        println!("{raw:?}");

        let baked = raw.bake(&reg).unwrap();

        assert_eq!(baked.get_component::<TestStatefulComponent>().unwrap().attr1, 1)
    }

    #[test]
    fn unbake_id() {
        let reg = reg_with_test_components();

        let mut ent_reg: Registry<Entity> = Registry::new();


        let mut ent = Entity::new();
        ent.require::<TestStatelessComponent>();
        let a = ent.require::<TestStatefulComponent>();
        a.attr1 = 1;

        let id = ent_reg.register(ent);


        let raw = EntityRaw::unbake(ent_reg.get(id).unwrap(), &reg).unwrap();

        println!("{raw:?}");

        let baked = raw.bake(&reg).unwrap();

        let orig = ent_reg.get(baked.get_id().unwrap()).unwrap();

        assert_eq!(orig.get_component::<TestStatefulComponent>().unwrap().attr1, 1)
    }

    #[test]
    fn bad_deserialize() {
        let reg = reg_with_test_components();

        let raw: EntityRaw = from_str(r#"EntityRaw(components: [("bad_tag", ())])"#).unwrap();

        let err = raw.bake(&reg).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn bad_serialize() {
        let reg = ModRegistry::new();

        let mut ent = Entity::new();
        ent.require::<TestStatelessComponent>();

        let err = EntityRaw::unbake(&ent, &reg).unwrap_err();

        println!("{err}")
    }
}