use serde::{Deserialize, Serialize};

use crate::component::stats::stat_modifier::StatModifier;


#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    base: i32,
    cap: i32,
    modifiers: Vec<Box<dyn StatModifier>>
}


impl Stat {
    pub fn new() -> Self {
        Stat {
            base: 0,
            cap: std::i32::MAX,
            modifiers: Vec::new()
        }
    }

    pub fn get(&self) -> i32 {
        self.modifiers.iter().fold(0, |a, b| 
            a + b.get_bonus()
        ) + self.base
    }

    pub fn get_cap(&self) -> i32 {
        self.modifiers.iter().fold(0, |a, b| 
            a + b.get_cap_bonus()
        ) + self.cap
    }

    pub fn set(&mut self, value: i32) {
        self.base = value;
    }

    pub fn set_cap(&mut self, value: i32) {
        self.cap = value;
    }

    pub fn add_modifier(&mut self, modifier: Box<dyn StatModifier>) {
        self.modifiers.push(modifier);
    }
}