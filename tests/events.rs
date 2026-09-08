use std::any::{Any, TypeId};

use modular_rogue::{component::component::{Component, Handles, dispatch}, entity::entity::Entity, event::event::Event};
use serde::{Deserialize, Serialize};

struct ConstEvent {
    pub handled: bool
}
impl Event for ConstEvent {}

#[derive(Serialize, Deserialize)]
struct ConstHandler {}
#[typetag::serde]
impl Component for ConstHandler {
    fn get_handlers(&self) -> Vec<(std::any::TypeId, fn(&mut dyn Component, &mut dyn Any))> {
        vec![
            (TypeId::of::<ConstEvent>(), dispatch::<ConstHandler, ConstEvent>)
        ]
    }
}
impl Handles<ConstEvent> for ConstHandler {
    fn handle(&mut self, e: &mut ConstEvent) {
        e.handled = true;
    }
}

#[test]
fn const_event() {
    let mut ent = Entity::new();
    ent.attach(Box::new(ConstHandler {}));
    let mut e = ConstEvent {handled: false};
    ent.handle(&mut e);
    assert_eq!(e.handled, true);
}