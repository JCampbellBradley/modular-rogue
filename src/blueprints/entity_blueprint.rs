
use ron::Value::{Map, Seq};
use serde::Deserialize;

use crate::{blueprints::blueprint::Blueprint, entities::entity::Entity, errors::blueprint_error::BlueprintError::{self, RonError}};

#[derive(Debug, Deserialize)]
pub struct EntityBlueprint {
    components: ron::Value
}

impl Blueprint<Entity> for EntityBlueprint {
    fn build(self) -> Result<Entity, BlueprintError> {
        let mut seq: Vec<ron::Value> = Vec::new();

        if let Map(components) = self.components {
            for (type_tag, contents) in components {
                let mut component =  ron::Map::new();

                component.insert("type", type_tag);
                component.insert("content", contents);

                seq.push(Map(component));
            }

            let mut ent_plan =  ron::Map::new();

            ent_plan.insert("components", Seq(seq));

            Map(ent_plan).into_rust::<Entity>()
                .map_err(RonError)
        } else {
            Err(BlueprintError::BadEntityBlueprintComponents { value: self.components })
        }
    }
}

#[cfg(test)]
mod tests {
    

    use crate::components::test_util::TestGetAttributesEvent;

    use super::*;

    #[test]
    fn test_load() {
        let bp: EntityBlueprint = ron::from_str(r#"EntityBlueprint(components: (_one_handles_component: (attr1: 5)))"#).unwrap();

        let mut ent = bp.build().unwrap();

        let mut e = TestGetAttributesEvent::default();

        ent.handle(&mut e);

        assert_eq!(e.attr1, 5)
    }

    #[test]
    fn bad_components() {
        let bp: EntityBlueprint = ron::from_str(r#"EntityBlueprint(components: 4)"#).unwrap();

        assert!(bp.build().is_err())
    }
}