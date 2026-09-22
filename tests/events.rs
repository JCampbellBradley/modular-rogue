use modular_rogue::{components::{component::{Component, Handles}}, entities::entity::Entity, events::event::Event};
use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

struct ConstEvent {
    pub handled: bool
}
impl Event for ConstEvent {}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug)]
#[handles(ConstEvent)]
struct ConstHandler {}

#[typetag::serde]
impl Component for ConstHandler {}

impl Handles<ConstEvent> for ConstHandler {
    fn handle(&mut self, e: &mut ConstEvent) {
        e.handled = true;
    }
}

#[test]
fn const_event() {
    let mut ent = Entity::new();
    let _ = ent.attach(Box::new(ConstHandler {}));
    let mut e = ConstEvent {handled: false};
    ent.handle(&mut e);
    assert_eq!(e.handled, true);
}