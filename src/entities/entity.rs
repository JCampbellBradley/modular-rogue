use serde::{Deserialize, Serialize};

use crate::{components::{component::Component, component_map::ComponentMap}, events::event::Event, worlds::{position::Position, registry::{Registerable, RegistryID}}};

pub type EntityId = RegistryID;

#[derive(Serialize, Deserialize, Default)]
pub struct Entity {
    id: EntityId,
    components: ComponentMap,
    pub zone_position: Position,
    pub position: Position
}

impl Entity{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attach(&mut self, component: Box<dyn Component>) {
        self.components.insert(component);
    }

    pub fn handle<T: Event + 'static>(&mut self, e: &mut T) {
        self.components.dispatch::<T>(e);
    }
}

impl Registerable for Entity {
    fn get_id(&self) -> RegistryID {
        self.id
    }

    fn set_id(&mut self, id: RegistryID) {
        self.id = id
    }
}