use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

use crate::{component::component::{Component, Handles}, event::stat_change_event::StatChangeEvent};

#[derive(Serialize, Deserialize, DynamicHandlers)]
#[handles(StatChangeEvent)]
struct Stats {

}

#[typetag::serde]
impl Component for Stats {}

impl Handles<StatChangeEvent> for Stats {
    fn handle(&mut self, e: &mut StatChangeEvent) {
        
    }
}