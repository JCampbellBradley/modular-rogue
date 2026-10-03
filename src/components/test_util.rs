use modular_rogue_macros::{DynamicHandlers, ModElement};
use serde::{Deserialize, Serialize};

use crate::{components::component::{Component, HandlesConst, HandlesMut}, events::event::{ConstEvent, Event, MutEvent}};


#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[handles()]
#[serde(default)]
#[component(name = "_zero_handles_component")]
pub struct TestZeroHandlesComponent {}
impl Component for TestZeroHandlesComponent {}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[handles(TestGetAttributesEvent, mut TestSetAttributesEvent)]
#[serde(default)]
#[component(name = "_one_handles_component")]
pub struct TestOneHandlesComponent {
    pub attr1: i32,
    pub attr2: i32,
}


impl Component for TestOneHandlesComponent {}

impl HandlesConst<TestGetAttributesEvent> for TestOneHandlesComponent {
    fn handle(&self, e: &mut TestGetAttributesEvent) {
        e.attr1 = self.attr1;
        e.attr2 = self.attr2;
    }
}

impl HandlesMut<TestSetAttributesEvent> for TestOneHandlesComponent{
    fn handle_mut(&mut self, e: &mut TestSetAttributesEvent) {
        self.attr1 = e.attr1;
        self.attr2 = e.attr2;
    }
}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[serde(default)]
#[component(name = "_zero_handles_component")]
pub struct DuplicateTagComponent {}
impl Component for DuplicateTagComponent {}

#[derive(Default)]
pub struct TestGetAttributesEvent {
    pub attr1: i32,
    pub attr2: i32
}
impl Event for TestGetAttributesEvent {}
impl ConstEvent for TestGetAttributesEvent {}


#[derive(Default)]
pub struct TestSetAttributesEvent {
    pub attr1: i32,
    pub attr2: i32
}
impl Event for TestSetAttributesEvent {}
impl MutEvent for TestSetAttributesEvent {}