use std::any::Any;
use std::any::TypeId;

use crate::component::component::Component;
use crate::event::attack_event::GenerateAttackEvent;
use crate::event::event::Event;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct AttackerComponent {
}

#[typetag::serde]
impl Component for AttackerComponent {
    fn handle(&mut self, e: &mut Box<dyn Event>) {
        if (*e).type_id() == TypeId::of::<GenerateAttackEvent>() {
            
        
        }
    }
}