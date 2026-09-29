use std::any::TypeId;


use crate::{components::{component::Component, component_map::ComponentMap}, errors::access_error::AccessError, events::event::{ConstEvent, MutEvent}, worlds::{position::Position, registry::{Registerable, RegistryID}}};

pub type EntityId = RegistryID;

#[derive(Debug)]
pub struct Entity {
    id: Option<EntityId>,
    component_map: ComponentMap,
    pub zone_position: Option<Position>,
    pub position: Option<Position>
}

impl Entity{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attach(&mut self, component: Box<dyn Component>) -> Result<(), AccessError<TypeId, Box<dyn Component>>> {
        self.component_map.insert(component)
    }

    pub fn handle<T: ConstEvent + 'static>(&self, e: &mut T) {
        self.component_map.dispatch::<T>(e);
    }

    pub fn handle_mut<T: MutEvent + 'static>(&mut self, e: &mut T) {
        self.component_map.dispatch_mut::<T>(e);
    }

    pub fn has_component(&self, t: TypeId) -> bool {
        self.component_map.contains_key(t)
    }

    pub fn components(&self) -> impl Iterator<Item=&dyn Component> {
        self.component_map.components()
    }
}

impl Registerable for Entity {
    fn get_id(&self) -> Option<RegistryID> {
        self.id
    }

    fn set_id(&mut self, id: RegistryID) {
        self.id = Some(id)
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity { 
            id: None, 
            component_map: ComponentMap::new(), 
            zone_position: None, 
            position: None 
        }
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {

}