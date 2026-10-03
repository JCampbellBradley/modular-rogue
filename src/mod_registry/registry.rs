use std::{any::TypeId, collections::HashMap};

use serde::de::DeserializeOwned;

use crate::{components::component::Component, errors::registry_error::RegistryError::{self}, mod_registry::mod_element::ModElement};



#[derive(Default, Debug)]
pub struct ModRegistry {
    components: HashMap<String, ComponentEntry>,
    tags: HashMap<TypeId, String>
}

impl ModRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_context<'a>(&'a mut self, mod_id: &'a str) -> ModContext<'a> {
        ModContext { mod_id, registry: self }
    }

    pub fn register_component(&mut self, tag: String, component_entry: ComponentEntry) -> Result<(), RegistryError> {
        if let Some(tag_first) = self.tags.remove(&component_entry.type_id) {
            return Err(RegistryError::DuplicateComponentType { id: component_entry.type_id, tag_first, tag_second: tag })
        }
        if let Some(entry_first) = self.components.remove(&tag) {
            return Err(RegistryError::DuplicateComponentTag { tag, entry_first, entry_second: component_entry })
        }

        self.tags.insert(component_entry.type_id, tag.clone());
        self.components.insert(tag, component_entry);

        Ok(())
    }

    pub fn serialize_component(&self, comp: &dyn Component) -> Result<(String, String), RegistryError> {
        let id = comp.type_id();

        if let Some(tag) = self.tags.get(&id) {
            let serialized = ron::to_string(comp)?;
            Ok((tag.clone(), serialized))
        } else {
            Err(RegistryError::SerializedUnknownType { id })
        }
    }

    pub fn deserialize_component(&self, tag: String, value: ron::Value) -> Result<Box<dyn Component>, RegistryError> {
        if let Some(entry) = self.components.get(&tag) {
            Ok((entry.deserialize_fn)(value)?)
        } else {
            Err(RegistryError::DeserializedUnknownTag { tag })
        }
    }

    pub fn get_tags(&self) -> impl Iterator<Item=&String> {
        self.tags.values()
    }
}

#[derive(Debug)]
pub struct ModContext<'a> {
    mod_id: &'a str,
    registry: &'a mut ModRegistry
}

impl ModContext<'_> {
    pub fn register_component<T: Component + ModElement + DeserializeOwned>(&mut self) -> Result<(), RegistryError> {
        let tag = format!("{}:{}", self.mod_id, T::NAME);

        let component_entry = ComponentEntry {
            owner: self.mod_id.to_string(),
            type_id: TypeId::of::<T>(),
            deserialize_fn: deserialize_as::<T>,
        };

        self.registry.register_component(tag, component_entry)?;
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct ComponentEntry {
    pub owner: String,
    pub type_id: TypeId,
    pub deserialize_fn: DeserializeFn
}

pub struct ModManifest {
    pub mod_id: &'static str,
    pub version: &'static str,
    pub dependencies: &'static [&'static str],
}

type DeserializeFn = fn(ron::Value) -> Result<Box<dyn Component>, ron::Error>;

fn deserialize_as<C: Component + DeserializeOwned>(v: ron::Value)
    -> Result<Box<dyn Component>, ron::Error>
{
    Ok(Box::new(v.into_rust::<C>()?))
}

#[cfg(test)]
mod tests {
    use std::{any::Any, ops::DerefMut};

use crate::components::test_util::{DuplicateTagComponent, TestOneHandlesComponent, TestZeroHandlesComponent};

use super::*;

    #[test]
    fn register_component() {
        let mut reg = ModRegistry::new();
        let mod_id = "mod_id";
        let mut ctx = reg.build_context(mod_id);

        ctx.register_component::<TestZeroHandlesComponent>().unwrap();

        let tags: Vec<&String> = reg.get_tags().collect();
        let tag = format!("{mod_id}:{}", TestZeroHandlesComponent::NAME);

        assert!(tags.contains(&&tag));
    }

    #[test]
    fn deserialize_component() {
        let mut reg = ModRegistry::new();
        let mod_id = "mod_id";
        let mut ctx = reg.build_context(mod_id);

        ctx.register_component::<TestOneHandlesComponent>().unwrap();

        let tag = format!("{mod_id}:{}", TestOneHandlesComponent::NAME);

        let value: ron::Value = ron::from_str("(attr2: 2)").unwrap();

        let mut component = reg.deserialize_component(tag, value).unwrap();
        let downcast = (component.deref_mut() as &dyn Any).downcast_ref::<TestOneHandlesComponent>().unwrap();
        
        assert_eq!(downcast.attr1, 0);
        assert_eq!(downcast.attr2, 2);
    }

    #[test]
    fn duplicate_component() {
        let mut reg = ModRegistry::new();
        let mod_id = "mod_id";
        let mut ctx = reg.build_context(mod_id);

        ctx.register_component::<TestZeroHandlesComponent>().unwrap();
        let err = ctx.register_component::<TestZeroHandlesComponent>().unwrap_err();

        println!("{err}");
    }

    #[test]
    fn duplicate_tag() {
        let mut reg = ModRegistry::new();
        let mod_id = "mod_id";
        let mut ctx = reg.build_context(mod_id);

        ctx.register_component::<TestZeroHandlesComponent>().unwrap();
        let err = ctx.register_component::<DuplicateTagComponent>().unwrap_err();

        println!("{err}");
    }

    #[test]
    fn ron_error() {
        let mut reg = ModRegistry::new();
        let mod_id = "mod_id";
        let mut ctx = reg.build_context(mod_id);

        ctx.register_component::<TestOneHandlesComponent>().unwrap();

        let tag = format!("{mod_id}:{}", TestOneHandlesComponent::NAME);
        let value: ron::Value = ron::from_str("(attr2: a)").unwrap();

        let err = reg.deserialize_component(tag, value).unwrap_err();

        println!("{err}");
    }

    #[test]
    fn unknown_tag() {
        let reg = ModRegistry::new();
        let mod_id = "mod_id";

        let tag = format!("{mod_id}:{}", TestOneHandlesComponent::NAME);

        let value: ron::Value = ron::from_str("(attr2: 2)").unwrap();

        let err = reg.deserialize_component(tag, value).unwrap_err();

        println!("{err}");
    }
}