use crate::{entity::entity::Entity, event::event::Event};

pub struct GenerateAttackEvent<'a> {
    pub defender: &'a Entity,
    pub attacker: &'a Entity
}

impl Event for GenerateAttackEvent<'_> {
    
}