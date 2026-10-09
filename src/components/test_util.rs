use std::any::TypeId;

use modular_rogue_macros::{DynamicHandlers, ModElement};
use serde::{Deserialize, Serialize};

use crate::{components::component::{Component, HandlesConst, HandlesMut, Priority}, events::event::{ConstEvent, Event, MutEvent}, mod_registry::mod_element::ModElement};


#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[handles(TestCounterEvent, TestPriorityEvent)]
#[serde(default)]
#[component(name = "_stateless_component")]
pub struct TestStatelessComponent {}
impl Component for TestStatelessComponent {
    fn get_priority(&self) -> Priority {
        return Priority::HIGHISH
    }
}

impl HandlesConst<TestCounterEvent> for TestStatelessComponent {
    fn handle(&self, e: &mut TestCounterEvent) {
        e.counter += 1;
    }
}

impl HandlesConst<TestPriorityEvent> for TestStatelessComponent {
    fn handle(&self, e: &mut TestPriorityEvent) {
        e.last_handler_id = Some(TypeId::of::<Self>());
    }
}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[handles(TestPriorityEvent, TestGetAttributesEvent, mut TestSetAttributesEvent)]
#[serde(default)]
#[component(name = "_stateful_component")]
pub struct TestStatefulComponent {
    pub attr1: i32,
    pub attr2: i32,
}


impl Component for TestStatefulComponent {
    fn get_priority(&self) -> Priority {
        return Priority::LOWISH
    }
}

impl HandlesConst<TestGetAttributesEvent> for TestStatefulComponent {
    fn handle(&self, e: &mut TestGetAttributesEvent) {
        e.attr1 = self.attr1;
        e.attr2 = self.attr2;
    }
}

impl HandlesMut<TestSetAttributesEvent> for TestStatefulComponent{
    fn handle_mut(&mut self, e: &mut TestSetAttributesEvent) {
        self.attr1 = e.attr1;
        self.attr2 = e.attr2;
    }
}

impl HandlesConst<TestPriorityEvent> for TestStatefulComponent {
    fn handle(&self, e: &mut TestPriorityEvent) {
        e.last_handler_id = Some(TypeId::of::<Self>());
    }
}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
#[serde(default)]
pub struct DuplicateTagComponent {}
impl Component for DuplicateTagComponent {}

impl ModElement for DuplicateTagComponent {
    const NAME: &'static str = "_stateless_component";
}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[serde(default, rename = "bad serde rename")]
pub struct BadSerdeRenameComponent {
    
}

impl Component for BadSerdeRenameComponent {}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default, ModElement)]
#[serde(default)]
pub struct RecursiveComponent {
    pub rec: Option<Box<RecursiveComponent>>
}

impl Component for RecursiveComponent {}

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


#[derive(Default)]
pub struct TestCounterEvent {
    pub counter: u32
}
impl Event for TestCounterEvent {}
impl ConstEvent for TestCounterEvent {}

#[derive(Default)]
pub struct TestPriorityEvent {
    pub last_handler_id: Option<TypeId>
}

impl Event for TestPriorityEvent {}
impl ConstEvent for TestPriorityEvent {}