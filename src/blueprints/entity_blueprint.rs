
use std::collections::HashMap;

use serde::Deserialize;

use crate::{entities::entity::Entity, errors::{bake_error::BakeError}, mod_registry::{bake::Bake, registry::ModRegistry}};

#[derive(Debug, Deserialize)]
pub struct EntityBlueprint<'a> {
    #[serde(borrow)]
    components: HashMap<String, &'a ron::value::RawValue>,
}

impl Bake<Entity> for EntityBlueprint<'_> {
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
    use crate::{blueprints::entity_blueprint::EntityBlueprint, components::test_util::TestStatefulComponent, mod_registry::{bake::Bake, test_util::reg_with_test_components}};

    #[test]
    fn bake() {
        let reg = reg_with_test_components();

        let ron_bp = r#"EntityBlueprint(components: {"test:_stateful_component": (attr2:3)})"#;
        let blueprint: EntityBlueprint = ron::from_str(ron_bp).unwrap();
        let ent = blueprint.bake(&reg).unwrap();

        let comp = ent.get_component::<TestStatefulComponent>().unwrap();

        assert_eq!(comp.attr2, 3);
        assert_eq!(comp.attr1, 0);
    }

    #[test]
    fn bake_bad_tag() {
        let reg = reg_with_test_components();

        let ron_bp = r#"EntityBlueprint(components: {"test:bad_tag": (attr2:3)})"#;
        let blueprint: EntityBlueprint = ron::from_str(ron_bp).unwrap();
        let err = blueprint.bake(&reg).unwrap_err();

        println!("{err}");
    }
}