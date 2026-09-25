use modular_rogue::{components::component::{Component, HandlesConst}, entities::entity::Entity, events::event::{ConstEvent, Event}};
use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

struct TestEvent {
    pub handled: bool
}
impl Event for TestEvent {}
impl ConstEvent for TestEvent {}

#[derive(Serialize, Deserialize, DynamicHandlers, Debug)]
#[handles(TestEvent)]
struct ConstHandler {}

#[typetag::serde]
impl Component for ConstHandler {}

impl HandlesConst<TestEvent> for ConstHandler {
    fn handle(&self, e: &mut TestEvent) {
        e.handled = true;
    }
}

#[test]
fn const_event() {
    let mut ent = Entity::new();
    let _ = ent.attach(Box::new(ConstHandler {}));
    let mut e = TestEvent {handled: false};
    ent.handle(&mut e);
    assert_eq!(e.handled, true);
}