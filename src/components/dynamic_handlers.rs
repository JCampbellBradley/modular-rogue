use std::any::{TypeId};

use crate::components::component::{DispatchFn};

pub trait DynamicHandlers {
    fn get_handlers(&self) -> Vec<(TypeId, DispatchFn)>;
}