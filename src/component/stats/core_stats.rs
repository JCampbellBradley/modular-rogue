use std::{collections::HashMap, fmt::Debug};

use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

use crate::{component::{component::{Component, Handles}, stats::stat::Stat}, event::stat_change_event::StatChangeEvent};


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
enum StatType {
    HP,

}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug)]
#[handles(StatChangeEvent)]
struct CoreStats {
    stat_map: HashMap<StatType, Stat>
}

#[typetag::serde]
impl Component for CoreStats {}

impl Handles<StatChangeEvent> for CoreStats {
    fn handle(&mut self, _e: &mut StatChangeEvent) {
        
    }
}