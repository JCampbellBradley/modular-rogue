use std::collections::{HashMap, HashSet};

use crate::{components::stats::stat::StatCategory, events::event::Event};


#[derive(Debug, Clone, Copy)]
pub struct StatCollected {
    pub base: i32,
    pub cap: i32
}

#[derive(Debug)]
pub struct StatCollectEvent<T: StatCategory> {
    pub wanted: HashSet<T>,
    pub stats: HashMap<T, Option<StatCollected>>
}

impl<T: StatCategory> Default for StatCollectEvent<T> {
    fn default() -> Self {
        StatCollectEvent { 
            wanted: HashSet::new(), 
            stats: HashMap::new() 
        }
    }
}

impl<T: StatCategory> StatCollectEvent<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn want(mut self, stat: T) -> Self {
        self.wanted.insert(stat);
        self
    }

    pub fn get(&mut self, stat: T) -> Option<&StatCollected> {
        match self.stats.get(&stat) {
            Some(Some(collected)) => Some(collected),
            _ => None
        }
    }
}

impl<T: StatCategory> Event for StatCollectEvent<T> {}