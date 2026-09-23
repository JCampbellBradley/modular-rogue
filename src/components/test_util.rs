use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

use crate::{components::component::{Component, Handles}, events::stat_change_event::StatChangeEvent};



#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
#[handles()]
pub struct TestZeroHandlesComponent {}

#[typetag::serde(name="_zero_handles_component")]
impl Component for TestZeroHandlesComponent {}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
#[handles(StatChangeEvent)]
#[serde(default)]
pub struct TestOneHandlesComponent {
    pub attr1: i32,
    pub attr2: i32,
}


#[typetag::serde(name="_one_handles_component")]
impl Component for TestOneHandlesComponent {}

impl Handles<StatChangeEvent> for TestOneHandlesComponent {
    fn handle(&mut self, _e: &mut StatChangeEvent) {
        
    }
}