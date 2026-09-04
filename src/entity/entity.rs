use serde::{Deserialize, Serialize};

use crate::{component::component::Component, entity::stats::{Stats}, event::event::Event};

#[derive(Serialize, Deserialize)]
pub struct Entity {
    stats: Stats,
    components: Vec<Box<dyn Component>>
}

impl Entity {
    pub fn new() -> Self {
        Entity { 
            stats : Stats::new(),
            components: Vec::new()
        }
    }

    pub fn add_component(&mut self, comp: Box<dyn Component>) {
        self.components.push(comp);
        self.components.sort_by(|a, b| 
            a.get_priority().cmp(&b.get_priority()));
    }

    pub fn handle(&mut self, e: &mut Box<dyn Event>) {
        for comp in &mut self.components {
            comp.handle(e);
        }
    }
}