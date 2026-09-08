use std::{any::{Any, TypeId}, collections::HashMap, ops::DerefMut};

use serde::{Deserialize, Serialize};

use crate::{component::component::Component, event::event::Event};


pub struct ComponentMap {
    component_map: HashMap<TypeId, Vec<(TypeId, fn(&mut dyn Component, &mut dyn Any))>>,
    components: HashMap<TypeId, Box<dyn Component>>
}

impl ComponentMap {
    pub fn new() -> Self {
        ComponentMap {
            component_map: HashMap::new(),
            components: HashMap::new()
        }
    }

    pub fn insert(&mut self, component: Box<dyn Component>) {
        let component_type = component.type_id();
        self.components.insert(component_type, component);
        let c = self.components.get_mut(&component_type).unwrap();

        for (t, f) in c.get_handlers().iter() {
            self.component_map.entry(*t)
                .or_insert(Vec::new())
                .push((component_type, *f))
        }

    }

    pub fn dispatch<T : Event + 'static>(&mut self, e: &mut T ) {
        for (t, f) in self.component_map.get(&TypeId::of::<T>()).unwrap() {
            let component = self.components.get_mut(t).unwrap().deref_mut();
            (f)(component, e);
        }
            
    }
}

impl Serialize for ComponentMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_none()
    }
}

impl<'de, 'a> Deserialize<'de> for ComponentMap {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(ComponentMap::new())
    }
}