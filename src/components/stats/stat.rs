use serde::{Deserialize, Serialize};

use crate::components::stats::stat_modifier::StatModifier;

#[derive(Deserialize)]
#[serde(untagged)]
enum StatRepr {
    Short(i32),
    Full {base: i32, modifiers: Vec<Box<dyn StatModifier>> }
}

pub trait StatCategory: Eq + std::hash::Hash {}

#[derive(Serialize, Debug, Default)]
pub struct Stat {
    base: i32,
    modifiers: Vec<Box<dyn StatModifier>>
}


impl Stat {
    pub fn new() -> Self {
        Stat::default()
    }

    pub fn get(&self) -> i32 {
        self.modifiers.iter().fold(0, |a, b| 
            a + b.get_bonus()
        ) + self.base
    }

    pub fn set(&mut self, value: i32) {
        self.base = value;
    }

    pub fn add_modifier(&mut self, modifier: Box<dyn StatModifier>) {
        self.modifiers.push(modifier);
    }
}

impl From<i32> for Stat {
    fn from(value: i32) -> Self {
        Stat{
            base: value, 
            ..Default::default()
        }
    }
}

impl<'de> Deserialize<'de> for Stat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let repr = StatRepr::deserialize(deserializer)?;
        let stat = match repr {
            StatRepr::Short(base) => Stat::from(base),
            StatRepr::Full {base, modifiers} => Stat {base, modifiers}
        };
        Ok(stat)
    }
}

#[cfg(test)]
mod tests {
    use crate::components::stats::test_util::TestModifier;

    use super::*;

    #[test]
    fn get() {
        let stat = Stat {base: 1, modifiers: Vec::new()};

        assert_eq!(stat.get(), 1);
    }

    #[test]
    fn set() {
        let mut stat = Stat {base: 1, modifiers: Vec::new()};
        stat.set(3);

        assert_eq!(stat.get(), 3);
    }

    #[test]
    fn add_1_modifier() {
        let mut stat = Stat::new();
        stat.add_modifier(Box::new(TestModifier {}));

        assert_eq!(stat.get(), TestModifier::BONUS);
    }

    #[test]
    fn add_2_modifier() {
        let mut stat = Stat::new();
        stat.add_modifier(Box::new(TestModifier {}));
        stat.add_modifier(Box::new(TestModifier {}));

        assert_eq!(stat.get(), TestModifier::BONUS * 2);
    }
}