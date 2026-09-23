use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

use crate::{components::component::{Component, Handles}, events::event::Event};



#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
#[handles()]
pub struct TestZeroHandlesComponent {}

#[typetag::serde(name="_zero_handles_component")]
impl Component for TestZeroHandlesComponent {}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
#[handles(GetAttributesEvent)]
#[serde(default)]
pub struct TestOneHandlesComponent {
    pub attr1: i32,
    pub attr2: i32,
}


#[typetag::serde(name="_one_handles_component")]
impl Component for TestOneHandlesComponent {}

impl Handles<GetAttributesEvent> for TestOneHandlesComponent {
    fn handle(&mut self, e: &mut GetAttributesEvent) {
        e.attr1 = self.attr1;
        e.attr2 = self.attr2;
    }
}

#[derive(Default)]
pub struct GetAttributesEvent {
    pub attr1: i32,
    pub attr2: i32
}
impl Event for GetAttributesEvent {}