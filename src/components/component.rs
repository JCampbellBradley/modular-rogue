use std::{any::{Any}, fmt::Debug};

use derive_more::{Add, Eq, PartialEq, Sub};

use crate::{components::dynamic_handlers::DynamicHandlers, events::event::{ConstEvent, MutEvent}};

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


pub type ConstDispatchFn = fn(&dyn Component, &mut dyn Any);
pub type MutDispatchFn = fn(&mut dyn Component, &mut dyn Any);

pub fn dispatch<C: HandlesConst<E> + 'static, E: ConstEvent + 'static>(component: &dyn Component, event: &mut dyn Any) {
    let handler = (component as &dyn Any).downcast_ref::<C>().unwrap();
    let downcast_event = event.downcast_mut::<E>().unwrap();
    handler.handle(downcast_event);
}

pub fn dispatch_mut<C: HandlesMut<E> + 'static, E: MutEvent + 'static>(component: &mut dyn Component, event: &mut dyn Any) {
    let handler = (component as &mut dyn Any).downcast_mut::<C>().unwrap();
    let downcast_event = event.downcast_mut::<E>().unwrap();
    handler.handle_mut(downcast_event);
}

pub trait HandlesConst<T: ConstEvent>: Component {
    fn handle(&self, e: &mut T);
}

pub trait HandlesMut<T: MutEvent>: Component {
    fn handle_mut(&mut self, e: &mut T);
}

pub trait Component: Any + DynamicHandlers + Debug + erased_serde::Serialize {
    fn get_priority(&self) -> Priority {
        Priority::DEFAULT
    }
    
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

erased_serde::serialize_trait_object!(Component);

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_sums_distinct() {
        let major_priorities = [Priority::LOW, Priority::LOWISH, Priority::DEFAULT, Priority::HIGHISH, Priority::HIGH];
        let biggest_nudge = Priority::NUDGE + Priority::NUDGE_SMALL + Priority::NUDGE_TINY + Priority::NUDGE_DIMINUTIVE;

        for i in 0..major_priorities.len()-1 {
            assert!(major_priorities[i].clone() + biggest_nudge < major_priorities[i+1] - biggest_nudge);
        }
    }
}