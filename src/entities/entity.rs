use serde::{Deserialize, Serialize};

use crate::{components::{component::Component, component_map::ComponentMap}, events::event::Event};

#[derive(Serialize, Deserialize, Default)]
pub struct Entity {
    components: ComponentMap,
    intrinsics: Vec<Box<dyn Component>>
}

impl Entity{
    pub fn new() -> Self {
        Entity { 
            components: ComponentMap::new(),
            intrinsics: Vec::new()
        }
    }

    pub fn attach(&mut self, component: Box<dyn Component>) {
        self.components.insert(component);
    }

    pub fn handle<T: Event + 'static>(&mut self, e: &mut T) {
        self.components.dispatch::<T>(e);
    }
}