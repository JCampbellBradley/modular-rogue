use std::any::{TypeId};

use crate::components::component::{ConstDispatchFn, MutDispatchFn};

pub trait DynamicHandlers {
    fn get_handlers(&self) -> Vec<(TypeId, ConstDispatchFn)>;
    fn get_handlers_mut(&self) -> Vec<(TypeId, MutDispatchFn)>;
}