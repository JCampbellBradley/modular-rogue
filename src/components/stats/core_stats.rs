use std::{collections::HashMap, fmt::Debug};

use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

use crate::{components::{component::{Component, Handles}, stats::stat::Stat}, events::stat_change_event::StatChangeEvent};


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
enum StatType {
    HP,
    
}

#[derive(Serialize, Deserialize, Default, DynamicHandlers, Debug)]
#[handles(StatChangeEvent)]
#[serde(default)]
struct CoreStats {
    stat_map: HashMap<StatType, Stat>
}

#[typetag::serde(name="core_stats")]
impl Component for CoreStats {}

impl Handles<StatChangeEvent> for CoreStats {
    fn handle(&mut self, _e: &mut StatChangeEvent) {
        
    }
}