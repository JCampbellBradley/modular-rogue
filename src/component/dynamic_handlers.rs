use std::any::{Any, TypeId};

use crate::component::component::Component;

pub trait DynamicHandlers {
    fn get_handlers(&self) -> Vec<(TypeId, fn(&mut dyn Component, &mut dyn Any))>;
}