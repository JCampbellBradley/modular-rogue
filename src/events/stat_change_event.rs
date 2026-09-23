use std::collections::HashMap;

use crate::{components::stats::stat::StatCategory, events::event::Event};

#[derive(Default, Debug, Clone, Copy)]
pub struct StatChangeImpulse {
    pub new_base: Option<i32>,
    pub new_cap: Option<i32>
}

impl StatChangeImpulse {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Debug)]
pub struct StatChangeEvent<T: StatCategory> {
    pub impulses: HashMap<T, StatChangeImpulse>
}

impl<T: StatCategory> StatChangeEvent<T> {
    pub fn new() -> Self {
        StatChangeEvent {
            impulses: HashMap::new()
        }
    }

    pub fn change_base(mut self, stat: T, new_base: i32) -> Self {
        self.impulses.entry(stat).or_default().new_base = Some(new_base);
        self
    }

    pub fn change_cap(mut self, stat: T, new_cap: i32) -> Self {
        self.impulses.entry(stat).or_default().new_cap = Some(new_cap);
        self
    }
}

impl<T: StatCategory> Event for StatChangeEvent<T> {}

#[cfg(test)]
mod tests {
    use crate::events::stat_change_event::StatChangeImpulse;

    #[test]
    fn initialize_impulse() {
        let impulse = StatChangeImpulse::new();

        assert!(impulse.new_base.is_none());
        assert!(impulse.new_cap.is_none());
    }
}