use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{components::{component::Component, component_map::ComponentMap}, events::event::Event, worlds::{position::Position, registry::{Registerable, RegistryID}}};

pub type EntityId = RegistryID;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Entity {
    id: EntityId,
    components: ComponentMap,
    pub zone_position: Option<Position>,
    pub position: Option<Position>
}

impl Entity{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attach(&mut self, component: Box<dyn Component>) -> Result<(), Box<dyn Error>> {
        self.components.insert(component)
    }

    pub fn handle<T: Event + 'static>(&mut self, e: &mut T) {
        self.components.dispatch::<T>(e);
    }
}

impl Registerable for Entity {
    fn get_id(&self) -> Option<RegistryID> {
        Some(self.id)
    }

    fn set_id(&mut self, id: RegistryID) {
        self.id = id
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity { 
            id: 0, 
            components: ComponentMap::new(), 
            zone_position: None, 
            position: None 
        }
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {

}