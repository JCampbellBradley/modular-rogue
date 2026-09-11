use std::{any::{Any, TypeId}, fmt::Debug};

use derive_more::{Add, Eq, PartialEq, Sub};

use crate::{components::dynamic_handlers::DynamicHandlers, events::event::Event};

#[derive(Add, Sub, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Priority(i8);

impl Priority {
    const LOW: Priority                 = Priority(-64);
    const LOWISH: Priority              = Priority(-32);
    const DEFAULT: Priority             = Priority(0);
    const HIGHISH: Priority             = Priority(32);
    const HIGH: Priority                = Priority(64);

    const NUDGE_DIMINUTIVE: Priority    = Priority(1);
    const NUDGE_TINY: Priority          = Priority(2);
    const NUDGE_SMALL: Priority         = Priority(4);
    const NUDGE: Priority               = Priority(8);
}


pub type DispatchFn = fn(&mut dyn Component, &mut dyn Any);

pub fn dispatch<C: Handles<E> + 'static, E: Event + 'static>(component: &mut dyn Component, event: &mut dyn Any) {
    let handler = (component as &mut dyn Any).downcast_mut::<C>().unwrap();
    let downcast_event = event.downcast_mut::<E>().unwrap();
    handler.handle(downcast_event);
}

pub trait Handles<T: Event>: Component {
    fn handle(&mut self, e: &mut T);
}

#[typetag::serde(tag = "type")]
pub trait Component: Any + DynamicHandlers + Debug {
    fn get_wanted_events(&self) -> Vec<TypeId> {
        self.get_handlers().iter()
            .map(|t| t.0)
            .collect()
    }

    fn get_priority(&self) -> Priority {
        Priority::DEFAULT
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::ops::Deref;

use modular_rogue_macros::DynamicHandlers;
    use serde::{Deserialize, Serialize};

    use crate::events::stat_change_event::StatChangeEvent;

    use super::*;


    #[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
    #[handles()]
    struct ZeroHandlesComponent {}

    #[typetag::serde]
    impl Component for ZeroHandlesComponent {}

    #[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
    #[handles(StatChangeEvent)]
    struct OneHandlesComponent {
        attr1: i32,
        attr2: i32,
    }

    #[typetag::serde]
    impl Component for OneHandlesComponent {}

    impl Handles<StatChangeEvent> for OneHandlesComponent {
        fn handle(&mut self, _e: &mut StatChangeEvent) {
            
        }
    }

    #[test]
    fn priority_sums_distinct() {
        let major_priorities = [Priority::LOW, Priority::LOWISH, Priority::DEFAULT, Priority::HIGHISH, Priority::HIGH];
        let biggest_nudge = Priority::NUDGE + Priority::NUDGE_SMALL + Priority::NUDGE_TINY + Priority::NUDGE_DIMINUTIVE;

        for i in 0..major_priorities.len()-1 {
            assert!(major_priorities[i].clone() + biggest_nudge < major_priorities[i+1] - biggest_nudge);
        }
    }

    #[test]
    fn no_wanted_events() {
        let component = ZeroHandlesComponent {};
        assert!(component.get_wanted_events().is_empty())
    }

    #[test]
    fn one_wanted_events() {
        let component = OneHandlesComponent::default();
        assert!(component.get_wanted_events().contains(&TypeId::of::<StatChangeEvent>()));
        assert_eq!(component.get_wanted_events().len(), 1);
    }

    #[test]
    fn serialize() {
        let component = Box::new(OneHandlesComponent{
            attr1: 1,
            attr2: 2
        });

        let serialized = serde_json::to_string(&*component as &dyn Component).unwrap();
        let deserialized: Box<dyn Component> = serde_json::from_str(&serialized).unwrap();
        let downcast = (deserialized.deref() as &dyn Any).downcast_ref::<OneHandlesComponent>().unwrap();

        assert_eq!(downcast.attr1, 1);
        assert_eq!(downcast.attr2, 2);
    }
}