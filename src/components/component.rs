use std::{any::{Any, TypeId}, fmt::Debug};

use derive_more::{Add, Eq, PartialEq, Sub};

use crate::{components::dynamic_handlers::DynamicHandlers, events::event::Event};

#[derive(Add, Sub, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Priority(i8);

impl Priority {
    pub const LOW: Priority                 = Priority(-64);
    pub const LOWISH: Priority              = Priority(-32);
    pub const DEFAULT: Priority             = Priority(0);
    pub const HIGHISH: Priority             = Priority(32);
    pub const HIGH: Priority                = Priority(64);

    pub const NUDGE_DIMINUTIVE: Priority    = Priority(1);
    pub const NUDGE_TINY: Priority          = Priority(2);
    pub const NUDGE_SMALL: Priority         = Priority(4);
    pub const NUDGE: Priority               = Priority(8);
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

    use crate::components::test_util::{GetAttributesEvent, TestOneHandlesComponent, TestZeroHandlesComponent};

    use super::*;

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
        let component = TestZeroHandlesComponent {};
        assert!(component.get_wanted_events().is_empty())
    }

    #[test]
    fn one_wanted_events() {
        let component = TestOneHandlesComponent::default();
        assert!(component.get_wanted_events().contains(&TypeId::of::<GetAttributesEvent>()));
        assert_eq!(component.get_wanted_events().len(), 1);
    }

    #[test]
    fn serialize() {
        let component = Box::new(TestOneHandlesComponent{
            attr1: 1,
            attr2: 2
        });

        let serialized = ron::to_string(&*component as &dyn Component).unwrap();
        let deserialized: Box<dyn Component> = ron::from_str(&serialized).unwrap();
        let downcast = (deserialized.deref() as &dyn Any).downcast_ref::<TestOneHandlesComponent>().unwrap();

        assert_eq!((*deserialized).type_id(), TypeId::of::<TestOneHandlesComponent>());
        assert_eq!(downcast.attr1, 1);
        assert_eq!(downcast.attr2, 2);
    }

    #[test]
    fn default_deserialization() {
        let serialized = r#"{"type": "_one_handles_component"}"#;
        let deserialized: Box<dyn Component> = ron::from_str(&serialized).unwrap();
        let downcast = (deserialized.deref() as &dyn Any).downcast_ref::<TestOneHandlesComponent>().unwrap();

        assert_eq!(downcast.attr1, 0);
        assert_eq!(downcast.attr2, 0);
    }
}