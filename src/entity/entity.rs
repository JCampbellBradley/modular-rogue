use std::any::{Any, TypeId};

use serde::{Deserialize, Serialize};

use crate::{component::component::{Component, ComponentMap}, entity::stats::Stats, event::event::Event};

#[derive(Serialize, Deserialize)]
pub struct Entity {
    stats: Stats,
    components: ComponentMap,
    intrinsics: Vec<Box<dyn Component>>
}

impl Entity{
    pub fn new() -> Self {
        Entity { 
            stats : Stats::new(),
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